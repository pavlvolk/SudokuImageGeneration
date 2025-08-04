/**
 * @class UIManager
 * @description Manages the user interface for the Sudoku game.
 * Handles grid generation, user interactions, mode switching,
 * background image management, and SVG/PDF generation for puzzle export.
 *
 * @requires sudoku.js
 * @requires jspdf
 * @requires svg2pdf
 */
class UIManager {
    /**
     * @description Initializes the UI Manager with all necessary DOM elements and state
     * @constructor
     */
    constructor() {
        // DOM Elements
        this.gridElement = document.getElementById("grid");
        this.backgroundDiv = document.getElementById('background');
        this.loadingOverlay = document.getElementById('loadingOverlay');
        this.partialSvg = document.getElementById('partialSvg');
        this.fullSvg = document.getElementById('fullSvg');
        this.changeCounterElement = document.getElementById("changeCounter");
        this.imageUpload = document.getElementById('imageUpload');
        this.rulesModal = document.getElementById("rulesModal");
        this.numberButton = document.getElementById("numbersBtn");
        this.createBtn = document.getElementById('createBtn');
        this.resetBtn = document.getElementById('resetBtn');
        this.rulesBtn = document.getElementById("rulesBtn");
        this.closeRulesBtn = document.getElementById("closeRulesBtn");
        this.rateDiffBtn = document.getElementById("rateDiffBtn");

        // Button Collections
        this.downloadButtons = {
            partial: document.getElementById("downloadBtn"),
            full: document.getElementById("download filled sudoku")
        };

        this.modeButtons = {
            mark: document.getElementById('markBtn'),
            input: document.getElementById('inputBtn')
        };

        this.sizeButtons = {
            4: document.getElementById("grid4"),
            6: document.getElementById("grid6"),
            9: document.getElementById("grid9")
        };

        // Game State
        this.sudoku = new Sudoku9();
        this.size = 9;
        this.isInGenerationMode = true;
        this.isInInsertingNumberMode = false;
        this.isPainting = false;
        this.backgroundImage = {
            url: null,
            hasImage: false
        };
        this.alreadyClicked = [];
        this.currentSolution = [];
        this.currentFullSolution = [];

        // Cell Tracking
        this.cellMap = new Map();
        this.indexMap = new Map();

        this.handleMouseDownCopy = this.handleGridMouseDown.bind(this);
        this.handleMouseOverCopy = this.handleGridMouseOver.bind(this);

        this.bindEventListeners();
        this.render();
        this.sizeButtons[9].classList.add('active-mode');
    }

    /**
     * @description Binds all necessary event listeners for the UI components
     * @private
     */
    bindEventListeners() {
        Object.entries(this.sizeButtons).forEach(([size, button]) => {
            button.addEventListener('click', () => this.handleSizeChange(parseInt(size)));
        });

        this.modeButtons.mark.addEventListener('click', () => this.handleModeChange(true));
        this.modeButtons.input.addEventListener('click', () => this.handleModeChange(false));
        document.addEventListener('mouseup', this.handleMouseUp.bind(this));
        this.downloadButtons.partial.addEventListener('click', () => this._downloadPDF(this.partialSvg, 'sudoku.pdf'));
        this.downloadButtons.full.addEventListener('click', () => this._downloadPDF(this.fullSvg, 'solution.pdf'));
        document.getElementById('uploadBtn').addEventListener('click', () => this.imageUpload.click());
        document.getElementById('removeBg').addEventListener('click', this.handleRemoveBackground.bind(this));
        this.imageUpload.addEventListener('change', this.handleImageUpload.bind(this));
        this.resetBtn.addEventListener('click', this.reset.bind(this));
        this.numberButton.addEventListener('click', this.handleNumberInput.bind(this));
        this.rulesBtn.addEventListener('click', this.showSudokuRules.bind(this));
        this.closeRulesBtn.addEventListener('click', this.closeRulesModal.bind(this));
        this.createBtn.addEventListener('click', this.handleCreateSudoku.bind(this));
        this.rateDiffBtn.addEventListener('click', this.handleRateDiff.bind(this));
    }

    /**
     * @description Resets the game state and clears the grid
     * @public
     */
    reset() {
        this.sudoku.reset();
        for (const cell of this.cellMap.keys()) {
            cell.classList.remove('marked', 'error');
            cell.innerHTML = '';
        }
        this.updateChangeCounter();
    }

    /**
     * @description Handles changing the grid size
     * @param {number} size - The new grid size (4, 6, or 9)
     * @public
     */
    handleSizeChange(size) {
        if (![4, 6, 9].includes(size)) return;

        switch (size) {
            case 4:
                this.sudoku = new Sudoku4();
                break;
            case 6:
                this.sudoku = new Sudoku6();
                break;
            case 9:
                this.sudoku = new Sudoku9();
                break;
        }

        this.size = size;
        Object.entries(this.sizeButtons).forEach(([btnSize, button]) => {
            button.classList.toggle('active-mode', parseInt(btnSize) === size);
        });

        this.render();
    }

    /**
     * @description Handles switching between generation and input modes
     * @param {boolean} isGenerationMode - True for generation mode, false for input mode
     * @public
     */
    handleModeChange(isGenerationMode) {
        if (this.isInGenerationMode === isGenerationMode) return;

        this.isInGenerationMode = isGenerationMode;
        this.reset();

        if (!isGenerationMode) {
            this.removeAllListeners();
            this.modeButtons.input.classList.toggle('active-mode');
            this.modeButtons.mark.classList.remove('active-mode');
            this._setupInputMode();
        } else {
            this.modeButtons.input.classList.remove('active-mode');
            this.modeButtons.mark.classList.toggle('active-mode');
            this.removeAllListeners();
            this.render();
        }
    }

    /**
     * @description Sets up input fields for number input mode
     * @private
     */
    _setupInputMode() {
        for (const [cell, index] of this.cellMap) {
            const input = document.createElement('input');
            input.setAttribute('maxlength', '1');
            input.setAttribute('type', 'text');

            const value = this.sudoku.getValueAt(index);
            if (value >= 2) {
                input.value = (value - 1).toString();
            }

            input.addEventListener('input', (e) => this.insertNumbersSubFunction(e, index));
            cell.appendChild(input);
        }
    }

    /**
     * @description Handles number input in a cell
     * @param {Event} e - Input event
     * @param {number} index - Cell index
     * @private
     */
    insertNumbersSubFunction(e, index) {
        const rowIndex = Math.floor(index / this.size);
        const colIndex = index % this.size;

        const colValues = this.sudoku.getColumn(colIndex).map(idx => this.sudoku.getValueAt(idx));
        const rowValues = this.sudoku.getRow(rowIndex).map(idx => this.sudoku.getValueAt(idx));
        const blockValues = this.sudoku.getBlockIndices(index).map(idx => this.sudoku.getValueAt(idx));

        const usedNumbers = new Set([
            ...colValues,
            ...rowValues,
            ...blockValues
        ].filter(v => v >= 2).map(v => v - 1));

        const allowedNumbers = Array.from(
            { length: this.size },
            (_, i) => i + 1
        ).filter(n => !usedNumbers.has(n));

        const allowedPattern = allowedNumbers.join('');
        const regex = new RegExp(`[^${allowedPattern}]`, 'g');
        const val = e.target.value.replace(regex, '').slice(0, 1);

        if (!val) {
            e.target.value = '';
            this.sudoku.setField(index, 0);
            return;
        }

        this.sudoku.setField(index, parseInt(val, 10) + 1);
    }

    /**
     * @description Shows the rules modal
     * @public
     */
    showSudokuRules() {
        this.rulesModal.style.display = "flex";
    }

    /**
     * @description Closes the rules modal
     * @public
     */
    closeRulesModal() {
        this.rulesModal.style.display = "none";
    }

    /**
     * @description Handles switching between number input modes
     * @public
     */
    handleNumberInput() {
        this.isInInsertingNumberMode = !this.isInInsertingNumberMode;
        this.numberButton.classList.toggle('active-mode', this.isInInsertingNumberMode);

        this.removeAllListeners();

        if (this.isInInsertingNumberMode) {
            this._setupInputMode();
        } else {
            this._restoreGenerationMode();
        }
    }

    /**
     * @description Restores the grid to generation mode
     * @private
     */
    _restoreGenerationMode() {
        for (const [cell, index] of this.cellMap) {
            cell.addEventListener('mousedown', this.handleMouseDownCopy);
            cell.addEventListener('mouseover', this.handleMouseOverCopy);

            const value = this.sudoku.getValueAt(index);
            cell.innerHTML = value >= 2 ? value - 1 : '';

            cell.style.color = 'black';
            cell.style.fontSize = '30px';
            cell.style.opacity = '1';
            cell.style.visibility = 'visible';
        }
    }

    /**
     * @description Removes all event listeners from cells
     * @private
     */
    removeAllListeners() {
        for (const cell of this.cellMap.keys()) {
            if (cell.firstChild) {
                cell.removeChild(cell.firstChild);
            }
            cell.removeEventListener('mousedown', this.handleMouseDownCopy);
            cell.removeEventListener('mouseover', this.handleMouseOverCopy);
        }
    }

    /**
     * @description Handles mouse down on grid cells
     * @param {MouseEvent} e - Mouse event
     * @private
     */
    handleGridMouseDown(e) {
        const cell = e.currentTarget;
        const index = this.cellMap.get(cell);

        cell.classList.toggle('marked');
        this.isPainting = true;
        this.sudoku.toggleField(index);
        this.alreadyClicked.push(cell);

        this.updateChangeCounter();
        e.target.innerHTML = '';
        this.updateMissingHints();
    }

    /**
     * @description Handles mouse over grid cells (for drag selection)
     * @param {MouseEvent} e - Mouse event
     * @private
     */
    handleGridMouseOver(e) {
        if (!this.isPainting) return;

        const cell = e.currentTarget;
        if (this.alreadyClicked.includes(cell)) return;

        const index = this.cellMap.get(cell);
        cell.classList.toggle('marked');
        this.alreadyClicked.push(cell);
        this.sudoku.toggleField(index);

        this.updateChangeCounter();
        e.target.innerHTML = '';
        this.updateMissingHints();
    }

    /**
     * @description Updates visual hints for invalid cells
     * @private
     */
    updateMissingHints() {
        const missingHints = this.sudoku.getHintGrid();
        for (let i = 0; i < this.size * this.size; i++) {
            const cell = this.indexMap.get(i);
            cell.classList.toggle('error', missingHints[i] !== 0);
        }
    }

    /**
     * @description Updates the counter showing marked/filled cells
     * @private
     */
    updateChangeCounter() {
        this.changeCounterElement.textContent = `${"Markierte Felder"}: ${this.sudoku.getChangeCount()}`;
    }

    /**
     * @description Handles mouse up event (end of drag selection)
     * @private
     */
    handleMouseUp() {
        this.alreadyClicked = [];
        this.isPainting = false;
    }

    /**
     * @description Handles image upload for background
     * @param {Event} event - Upload event
     * @public
     */
    async handleImageUpload(event) {
        const file = event.target.files[0];
        if (!file) return;

        const reader = new FileReader();
        reader.onload = async (e) => {
            this.backgroundImage.url = e.target.result;
            this.backgroundImage.hasImage = true;
            this.backgroundDiv.style.backgroundImage = `url('${this.backgroundImage.url}')`;

            if (this.currentSolution?.length > 0) {
                const [tempPartialSvg, tempFullSvg] = await Promise.all([
                    this._renderSVGToNewElement(this.currentSolution, this.currentFullSolution, false),
                    this._renderSVGToNewElement(this.currentSolution, this.currentFullSolution, true)
                ]);

                this.partialSvg.replaceWith(tempPartialSvg);
                this.fullSvg.replaceWith(tempFullSvg);

                this.partialSvg = tempPartialSvg;
                this.fullSvg = tempFullSvg;
            }
        };

        reader.readAsDataURL(file);
        event.target.value = '';
    }

    /**
     * @description Creates a new SVG element with rendered content
     * @param {Array} solution - Partial solution
     * @param {Array} fullSolution - Complete solution
     * @param {boolean} isFull - Whether to show full solution
     * @returns {Promise<SVGElement>} The rendered SVG element
     * @private
     */
    async _renderSVGToNewElement(solution, fullSolution, isFull) {
        const tempSvg = (isFull ? this.fullSvg : this.partialSvg).cloneNode(true);
        await this._renderSVG(solution, fullSolution, tempSvg, isFull);
        return tempSvg;
    }

    /**
     * @description Removes background image
     * @public
     */
    async handleRemoveBackground() {
        const tempPartialSvg = document.createElement('svg');
        const tempFullSvg = document.createElement('svg');

        this.backgroundDiv.style.backgroundImage = 'none';
        this.backgroundImage.hasImage = false;
        this.backgroundImage.url = null;

        if (this.currentSolution?.length > 0) {
            await this._renderSVG(this.currentSolution, this.currentFullSolution, tempPartialSvg);
            await this._renderSVG(this.currentSolution, this.currentFullSolution, tempFullSvg, true);

            this.partialSvg.innerHTML = tempPartialSvg.innerHTML;
            this.fullSvg.innerHTML = tempFullSvg.innerHTML;
            this.partialSvg.style.display = "block";
        }
    }

    /**
     * @description Handles creation of new Sudoku puzzle
     * @public
     */
    async handleCreateSudoku() {
        if (this.sudoku.size === 9 && this.sudoku.changes < 17 && this.isInGenerationMode) {
            alert("For a 9x9 grid, please mark at least 17 fields.");
            return;
        }
        if (this.sudoku.changes === 0) {
            alert("Please mark some fields or input some numbers first.");
            return;
        }

        this.loadingOverlay.style.display = 'flex';
        this._toggleDownloadButtons(false);

        try {
            const response = await fetch('http://localhost:8080/api/process-tuple', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    data: this.sudoku.getList(),
                    length: this.sudoku.size * this.sudoku.size,
                    markingmode: this.isInGenerationMode
                })
            });

            if (!response.ok) throw new Error(`Server error: ${response.statusText}`);

            const result = await response.json();

            if (!result.hassolution) {
                alert("No matching Sudoku was found in our database.");
            } else {
                this.currentSolution = result.data;
                this.currentFullSolution = result.solution;
                await this._renderSVG(result.data, result.solution, this.partialSvg);
                await this._renderSVG(result.data, result.solution, this.fullSvg, true);
                this.partialSvg.style.display = "block";
                this._toggleDownloadButtons(true);
            }
        } catch (error) {
            alert("An error occurred while creating the Sudoku. Make sure your local server is running. Error: " + error.message);
            console.error("Error creating Sudoku:", error);
        } finally {
            this.loadingOverlay.style.display = 'none';
        }
    }

    async handleRateDiff(){
        if (this.sudoku.size === 9 && this.sudoku.changes < 17 && this.isInGenerationMode) {
            alert("For a 9x9 grid, please mark at least 17 fields.");
            return;
        }
        if (this.sudoku.changes === 0) {
            alert("Please mark some fields or input some numbers first.");
            return;
        }

        this.loadingOverlay.style.display = 'flex';
        this._toggleDownloadButtons(false);

        try{
            //TODO
        }catch(error){
            //TODO
        }
    }

    /**
     * @description Renders the initial grid
     * @public
     */
    render() {
        this.reset();
        this._drawGrid();
        this._toggleDownloadButtons(false);
        this.partialSvg.style.display = "none";
    }

    /**
     * @description Draws the Sudoku grid
     * @private
     */
    _drawGrid() {
        this.updateChangeCounter();
        const [blockRows, blockCols] = this._getBlockSize();

        this.gridElement.style.gridTemplateColumns = `repeat(${this.size}, 1fr)`;
        this.gridElement.innerHTML = '';

        for (let row = 0; row < this.size; row++) {
            for (let col = 0; col < this.size; col++) {
                const cell = document.createElement('div');
                cell.classList.add('cell');

                if (row % blockRows === 0) cell.classList.add('thick-top');
                if (col % blockCols === 0) cell.classList.add('thick-left');
                if (col === this.size - 1) cell.classList.add('thick-right');
                if (row === this.size - 1) cell.classList.add('thick-bottom');

                cell.addEventListener('mousedown', this.handleMouseDownCopy);
                cell.addEventListener('mouseover', this.handleMouseOverCopy);

                this.gridElement.appendChild(cell);
                const index = (row * this.size) + col;
                this.cellMap.set(cell, index);
                this.indexMap.set(index, cell);
            }
        }
    }

    /**
     * @description Toggles visibility of download buttons
     * @param {boolean} show - Whether to show the buttons
     * @private
     */
    _toggleDownloadButtons(show) {
        const display = show ? 'block' : 'none';
        this.downloadButtons.partial.style.display = display;
        this.downloadButtons.full.style.display = display;
    }

    /**
     * @description Gets the block size for the current grid size
     * @returns {[number, number]} Block dimensions [rows, cols]
     * @private
     */
    _getBlockSize() {
        switch (this.size) {
            case 4: return [2, 2];
            case 6: return [2, 3];
            case 9: return [3, 3];
            default: return [1, 1];
        }
    }

    /**
     * @description Renders SVG representation of the Sudoku
     * @param {Array} data - Current puzzle state
     * @param {Array} solution - Complete solution
     * @param {SVGElement} svgElement - Target SVG element
     * @param {boolean} [isFullSolution=false] - Whether to show complete solution
     * @private
     */
    async _renderSVG(data, solution, svgElement, isFullSolution = false) {
        const cellSize = 50;
        const totalSize = this.size * cellSize;

        svgElement.innerHTML = '';
        svgElement.setAttribute('viewBox', `0 0 ${totalSize} ${totalSize}`);
        svgElement.setAttribute('width', totalSize);
        svgElement.setAttribute('height', totalSize);

        svgElement.innerHTML = `<rect x="0" y="0" width="${totalSize}" height="${totalSize}" fill="white"/>`;

        if (this.backgroundImage.hasImage) {
            const grayDataURL = await this._convertImageToGrayscale(this.backgroundImage.url);
            svgElement.innerHTML += `
                <defs>
                    <pattern id="bgPattern" patternUnits="userSpaceOnUse" width="${totalSize}" height="${totalSize}">
                        <image href="${grayDataURL}" width="${totalSize}" height="${totalSize}" preserveAspectRatio="xMidYMid slice"></image>
                    </pattern>
                </defs>
                <rect x="0" y="0" width="${totalSize}" height="${totalSize}" fill="url(#bgPattern)"></rect>
            `;
        }

        for (let i = 0; i < data.length; i++) {
            const row = Math.floor(i / this.size);
            const col = i % this.size;
            const x = col * cellSize;
            const y = row * cellSize;

            const fill = (data[i] !== 0 && !this.backgroundImage.hasImage)
                ? 'rgba(128, 128, 128, 0.3)'
                : 'rgba(255, 255, 255, 0.5)';

            svgElement.innerHTML += `
                <rect x="${x}" y="${y}" 
                      width="${cellSize}" height="${cellSize}" 
                      fill="${fill}" stroke="black" stroke-width="1"></rect>
            `;

            if (data[i] !== 0 || isFullSolution) {
                svgElement.innerHTML += `
                    <text x="${x + cellSize / 2}" y="${y + cellSize * 0.75}" 
                          font-size="${cellSize * 0.8}px" text-anchor="middle" 
                          fill="black">${solution[i]}</text>
                `;
            }
        }

        const [blockRows, blockCols] = this._getBlockSize();
        for (let i = 1; i < this.size; i++) {
            if (i % blockRows === 0) {
                svgElement.innerHTML += `
                    <line x1="0" y1="${i * cellSize}" 
                          x2="${totalSize}" y2="${i * cellSize}" 
                          stroke="black" stroke-width="3"></line>
                `;
            }
            if (i % blockCols === 0) {
                svgElement.innerHTML += `
                    <line x1="${i * cellSize}" y1="0" 
                          x2="${i * cellSize}" y2="${totalSize}" 
                          stroke="black" stroke-width="3"></line>
                `;
            }
        }

        svgElement.innerHTML += `
            <rect x="0" y="0" width="${totalSize}" height="${totalSize}" 
                  fill="none" stroke="black" stroke-width="5"></rect>
        `;
    }

    /**
     * @description Downloads SVG as PDF
     * @param {SVGElement} svgElement - SVG to convert
     * @param {string} filename - Output filename
     * @private
     */
    async _downloadPDF(svgElement, filename) {
        if (!window.jspdf || !window.svg2pdf) {
            alert("PDF libraries are not loaded. Cannot download PDF.");
            console.error("jspdf or svg2pdf not found on window object.");
            return;
        }

        const { jsPDF } = window.jspdf;
        const { svg2pdf } = window.svg2pdf;

        const size = this.size * 50;
        const pdf = new jsPDF({ unit: 'pt', format: [size, size] });

        svgElement.style.display = "block";
        await svg2pdf(svgElement, pdf, { x: 0, y: 0, scale: 1 });
        pdf.save(filename);
    }

    /**
     * @description Converts image to grayscale
     * @param {string} src - Image source URL
     * @returns {Promise<string>} Grayscale image data URL
     * @private
     */
    _convertImageToGrayscale(src) {
        return new Promise((resolve, reject) => {
            const img = new Image();
            img.crossOrigin = "anonymous";

            img.onload = () => {
                const canvas = document.createElement("canvas");
                canvas.width = img.width;
                canvas.height = img.height;

                const ctx = canvas.getContext("2d");
                ctx.drawImage(img, 0, 0);

                const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
                const data = imageData.data;

                for (let i = 0; i < data.length; i += 4) {
                    const avg = 0.299 * data[i] + 0.587 * data[i + 1] + 0.114 * data[i + 2];
                    data[i] = data[i + 1] = data[i + 2] = avg;
                }

                ctx.putImageData(imageData, 0, 0);
                resolve(canvas.toDataURL());
            };

            img.onerror = reject;
            img.src = src;
        });
    }
}

// Initialize the UI manager once the DOM is loaded
window.addEventListener('DOMContentLoaded', () => {
    new UIManager();
});
