let changes = 0;
let currentSize = 9;
let gridState = [];
let listofCells = [];
let insertNumberstrue;
let markMode = true;
let isMouseDown = false;
let alreadyclicked = [];
document.getElementById("grid9").classList.add('active-mode')
document.getElementById("markBtn").classList.add('active-mode')
const gridElement = document.getElementById("grid");
const modeSelector = document.getElementById("modeSelector");
const serverUrl = "http://localhost:8080";
const { jsPDF } = window.jspdf;
const { svg2pdf } = window.svg2pdf;
const partialSvg = document.getElementById('partialSvg');
const fullSvg = document.getElementById('fullSvg');
const uploadBtn = document.getElementById('uploadBtn');
const imageUpload = document.getElementById('imageUpload');
const removeBtn = document.getElementById('removeBg');
const backgroundDiv = document.getElementById('background');
const rateDiffBtn = document.getElementById('rateDiffBtn');
let hasBackgroundImage = false;
let imageURL;


rateDiffBtn.addEventListener('click', (e) => {

});

/*
Entfernt das Hintergrundbild vom Sudoku
 */
removeBtn.addEventListener('click', () => {
    backgroundDiv.style.backgroundImage = 'none';
    hasBackgroundImage = false;
});


uploadBtn.addEventListener('click', () => {
    imageUpload.click();  // öffnet den Datei-Dialog
});

document.addEventListener('mouseup', () => {isMouseDown = false;
alreadyclicked = [];});
/*
stellt Feld für Upload zur Verfügung, um das Bild als Hintergrund zu benutzen
 */
imageUpload.addEventListener('change', (event) => {
    const file = event.target.files[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = function(e) {
        backgroundDiv.style.backgroundImage = `url('${e.target.result}')`;
        imageURL = e.target.result;
        hasBackgroundImage = true;
    };
    reader.readAsDataURL(file);

    event.target.value = '';
});


function getBlockSize(size) {
    if (size === 4) return [2, 2];
    if (size === 6) return [2, 3];
    if (size === 9) return [3, 3];
    return [1, 1];
}


function createGrid(size) {
    listofCells = [];
    const [blockRows, blockCols] = getBlockSize(size);
    gridElement.style.gridTemplateColumns = `repeat(${size}, 1fr)`;
    gridElement.innerHTML = '';
    gridState = Array(size * size).fill(0);

    for (let row = 0; row < size; row++) {
        for (let col = 0; col < size; col++) {
            const cell = document.createElement('div');
            cell.classList.add('cell');

            if (row % blockRows === 0) cell.classList.add('thick-top');     
            if (col % blockCols === 0) cell.classList.add('thick-left');
            if (col === size - 1) cell.classList.add('thick-right');
            if (row === size - 1) cell.classList.add('thick-bottom');

            const index = row * size + col;
            cell.dataset.index = index;
            listofCells.push([cell, index]);


            if (markMode === false) {
                const input = document.createElement('input');
                input.setAttribute('maxlength', '1');
                input.setAttribute('type', 'text');
                input.addEventListener('input', (e) => {
                    document.querySelectorAll(".cell").forEach(cell => {
                        cell.classList.remove(`error`);
                    });
                    const regex = new RegExp(`[^1-${currentSize}]`, 'g');
                    const val = e.target.value.replace(regex, '').slice(0, 1);
                    e.target.value = val;
                    gridState[index] = val ? parseInt(val, 10) : 0;  // ← Convert to number
                    if (val){
                        changes ++
                    }else{
                        changes --
                    }
                    updateChangeCounter();
                    dynamicErrors();
                });
                cell.appendChild(input);
            } else {
                cell.addEventListener('mouseover', markingModeFunction);
                cell.addEventListener('mousedown', markingModeFunctionSingleClick);
            }

            gridElement.appendChild(cell);
        }
    }


}
function markingModeFunction(e) {
    const cell = e.currentTarget;
    const index = Number(cell.dataset.index);
    if (isMouseDown && !(alreadyclicked.includes(index))) {
        alreadyclicked.push(index);
        const marked = cell.classList.toggle('marked');
        gridState[index] = marked ? 1 : 0;
        changes += marked ? 1 : -1;
        updateChangeCounter();
        document.getElementById('numbersBtn').style.display =
            (changes >= 1 ? 'block' : 'none');
        dynamicErrors();
        updateChangeCounter();
        cell.innerHTML = '';
    }
}
function markingModeFunctionSingleClick(e) {
    const cell = e.currentTarget;
    const index = Number(cell.dataset.index);
    if(!(alreadyclicked.includes(index))) {
        alreadyclicked.push(index);
        isMouseDown = true;
        const marked = cell.classList.toggle('marked');
        gridState[index] = marked ? 1 : 0;
        changes += marked ? 1 : -1;
        updateChangeCounter();
        document.getElementById('numbersBtn').style.display =
            (changes >= 1 ? 'block' : 'none');
        dynamicErrors();
        updateChangeCounter();
        cell.innerHTML = '';
    }
}

function dynamicErrors(){
    const tupel = {
        size: currentSize,
        mode: markMode,
        state: [...gridState]
    };
    const errors = validTupel(tupel);
    highlightErrors(errors,tupel);
}



/*
Updatet die Anzahl der markierten Felder
 */
function updateChangeCounter() {
    document.getElementById("changeCounter").textContent = `Markierte Felder: ${changes}`;
}


/*
Verändert die Grid Size, falls ein Button ausgewählt wurde, welcher nicht aktiv ist
Ruft danach Reset Grid auf
 */
function setGridSize(size) {
    if(currentSize === size){
        return;
    }

    document.getElementById("grid4").classList.remove('active-mode')
    document.getElementById("grid9").classList.remove('active-mode')
    document.getElementById("grid6").classList.remove('active-mode')
    if (size === 4){
        document.getElementById("grid4").classList.add('active-mode')
    }else if (size === 6){
        document.getElementById("grid6").classList.add('active-mode')
    }else if (size === 9){
        document.getElementById("grid9").classList.add('active-mode')
    }

    currentSize = size;
    resetGrid();
}

/*
Wird immer dann ausgeführt, wenn der Modus geändert wird
Wenn der bereits aktivierte Button angeklickt wird, ändert sich nichts
Ändert die Hervorhebung der Button und ruft Originalfunktion auf

 */
function selectMode(mode) {
    if(modeSelector.value === mode){
        return;
    }
    modeSelector.value = mode;

    document.getElementById('markBtn').classList.remove('active-mode');
    document.getElementById('inputBtn').classList.remove('active-mode');

    if (mode === 'mark') {
        document.getElementById('markBtn').classList.add('active-mode');
    } else if (mode === 'input') {
        document.getElementById('inputBtn').classList.add('active-mode');
    }
    changeMode();
}

/*
Setzt den Wert für markMode, welcher später ans Backend übergeben wird
Ruft anschließend Reset Grid auf
 */
function changeMode() {
    if (modeSelector.value === "input"){
        markMode = false;
    }else if (modeSelector.value === "mark"){
        markMode = true
    }
    resetGrid();
}

/*
resetet Änderungszahl
Verstekt die Download-Buttons, da nicht mehr am aktuellen Sudoku gearbeitet wird
 */
function resetGrid() {
    changes = 0;
    updateChangeCounter();
    hideDownloadButton();
    partialSvg.style.display = "none";
    createGrid(currentSize);
}


/*
Blendet die beiden Download-Button ein, immer dann wenn ein Sudoku generiert wurde
 */
function showDownloadButton() {
    document.getElementById("downloadBtn").style.display = "block";
    document.getElementById("download filled sudoku").style.display = "block";
}
/*
Lässt die Button verschwinden, immer bei Reset
 */
function hideDownloadButton() {
    document.getElementById("downloadBtn").style.display = "none";
    document.getElementById("download filled sudoku").style.display = "none";
}

function createSudoku() {
    partialSvg.style.display = "none";
    hideDownloadButton();


    const tupel = {
        size: currentSize,
        mode: markMode,
        state: [...gridState]
    };

    let errors = validTupel(tupel);
    if (changes <17 && tupel.size === 9){
        alert("markiere mindestens 17 Elemente")
    }else
    if (errors.length !== 0){
        alert("markiere Elemente bis nichts mehr rot ist")
    }else {
        document.getElementById('loadingOverlay').style.display = 'flex';

        console.log("Sudoku-Tupel:", tupel);
        document.getElementById('loadingOverlay').style.display = 'flex';
        document.getElementById("downloadBtn").style.display = "none";
        const payload = {
            data: gridState,
            length: currentSize * currentSize,
            markingmode: markMode,
        };
        fetch(`${serverUrl}/api/process-tuple`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(payload)
        })
            .then(response => response.json())
            .then(result => {
                if(!result.hassolution) {
                    alert("In unserer Datenbank wurde kein passendes Sukoku gefunden!");
                }else {
                    renderSudoku(result.data, result.solution, partialSvg);
                    renderSudoku(result.data, result.solution, fullSvg);
                    partialSvg.style.display = "block";
                    showDownloadButton();
                }
            })
            .catch(error => {
                alert("Es ist ein Fehler aufgetreten: " + error.message);
                console.error("Error:", error);
            })
            .finally(() => {
                document.getElementById("loadingOverlay").style.display = "none";
            });

        console.log("Sudoku-Tupel:", tupel);
        document.getElementById("downloadBtn").style.display = "none";
    }

}


function validTupel(tupel) {
    const size = tupel.size;
    const values = tupel.state;
    const [blockRows, blockCols] = getBlockSize(size);
    const errors = [];

    // === MARK-MODUS PRÜFUNG ===
    if (tupel.mode === true) {
        const hasContent = values.some(v => v === 1);
        if (!hasContent) errors.push("Keine Felder markiert!");

        checkRowColDistribution(values, size, errors);
    }

    // === INPUT-MODUS PRÜFUNG ===
    if (tupel.mode === false) {
        const getCoordinates = (index) => {
            return `(${Math.floor(index / size) + 1}, ${index % size + 1})`;
        };

        const findDuplicates = (group, groupType, groupIndex) => {
            const seen = {};
            for (let i = 0; i < group.length; i++) {
                const val = group[i];
                if (val !== "0" && val !== "") {
                    if (seen[val] !== undefined && val !== 0) {
                        const firstIndex = seen[val];
                        const secondIndex = i;
                        let x = secondIndex % size + 1
                        let y = firstIndex % size + 1
                        if (groupType === "Zeile"){
                            errors.push(["E",(groupIndex + 1)*size + y - (size +1), (groupIndex + 1)*size + x -(size+1)])
                        }else if (groupType === "Spalte"){
                            errors.push(["E",y * size + groupIndex +1 - (size +1),x * size + groupIndex +1 - (size + 1)]);
                        }
                        //errors.push(`Doppelte Zahl '${val}' in ${groupType} ${groupIndex + 1} an Positionen ${getCoordinates(firstIndex)} und ${getCoordinates(secondIndex)}`);
                    } else {
                        seen[val] = i;
                    }
                }
            }
        };

        // Zeilen prüfen
        for (let row = 0; row < size; row++) {
            const rowVals = [];
            for (let col = 0; col < size; col++) {
                const index = row * size + col;
                rowVals.push(values[index]);
            }
            findDuplicates(rowVals, "Zeile", row);
        }

        // Spalten prüfen
        for (let col = 0; col < size; col++) {
            const colVals = [];
            for (let row = 0; row < size; row++) {
                const index = row * size + col;
                colVals.push(values[index]);
            }
            findDuplicates(colVals, "Spalte", col);
        }

        // Blöcke prüfen
        for (let blockRow = 0; blockRow < size; blockRow += blockRows) {
            for (let blockCol = 0; blockCol < size; blockCol += blockCols) {
                const blockVals = [];
                const blockIndexes = [];
                for (let r = 0; r < blockRows; r++) {
                    for (let c = 0; c < blockCols; c++) {
                        const row = blockRow + r;
                        const col = blockCol + c;
                        const index = row * size + col;
                        blockVals.push(values[index]);
                        blockIndexes.push(index);
                    }
                }
                const seen = {};
                for (let i = 0; i < blockVals.length; i++) {
                    const val = blockVals[i];
                    if (val !== "0" && val !== "") {
                        if (seen[val] !== undefined && val !== 0) {
                            const firstIndex = blockIndexes[seen[val]];
                            const secondIndex = blockIndexes[i];
                            //errors.push(`Doppelte Zahl '${val}' im Block (${Math.floor(blockRow/blockRows)+1}, ${Math.floor(blockCol/blockCols)+1}) an ${getCoordinates(firstIndex)} und ${getCoordinates(secondIndex)}`);
                            errors.push(["E",coordinatesToList(size,getCoordinates(firstIndex)),coordinatesToList(size,getCoordinates(secondIndex))]);
                        } else {
                            seen[val] = i;
                        }
                    }
                }
            }
        }

        // Verteilung prüfen
        checkRowColDistribution(values, size, errors);
    }

    return errors;
}

function coordinatesToList(size,coordinates){
    const [row, col] = coordinates
        .slice(1, -1)
        .split(',')
        .map(s => parseInt(s));
    return (row - 1) * size + col -1
}

function checkRowColDistribution(values, size, errors) {
    let blockSize = size === 9 ? 3 : size === 6 || 4 ? 2 : 1;

    // Zeilenblöcke prüfen
    for (let i = 0; i < size; i += blockSize) {
        let filledRows = 0;
        let emptyRows = []; // Sammeln, welche Zeilen leer sindA
        for (let r = i; r < i + blockSize; r++) {
            const hasEntry = values.slice(r * size, (r + 1) * size).some(v => v !== 0 && v !== "0" && v !== "");
            if (hasEntry) {
                filledRows++;
            } else {
                emptyRows.push(r + 1); // Zeilennummer speichern (beginnend bei 1)
            }
        }
        if ((blockSize === 3 && filledRows < 2) || (blockSize === 2 && filledRows < 1)) {
            //errors.push(`Zu wenig markierte/ausgefüllte Zeilen im Block ${i / blockSize + 1}: fehlende Zeilen ${emptyRows.join(", ")}`);
            errors.push(["Z",emptyRows]);
        }
    }

    // Spaltenblöcke prüfen
    if (size === 6){
        blockSize = 3
    }
    for (let i = 0; i < size; i += blockSize) {
        let filledCols = 0;
        let emptyCols = []; // Sammeln, welche Spalten leer sind
        for (let c = i; c < i + blockSize; c++) {
            let hasEntry = false;
            for (let r = 0; r < size; r++) {
                const index = r * size + c;
                if (values[index] !== 0 && values[index] !== "0" && values[index] !== "") {
                    hasEntry = true;
                    break;
                }
            }
            if (hasEntry) {
                filledCols++;
            } else {
                emptyCols.push(c + 1); // Spaltennummer speichern (beginnend bei 1)
            }
        }
        if ((blockSize === 3 && filledCols < 2) || (blockSize === 2 && filledCols < 1)) {
            //errors.push(`Zu wenig markierte/ausgefüllte Spalten im Block ${i / blockSize + 1}: fehlende Spalten ${emptyCols.join(", ")}`);
            errors.push(["S",emptyCols])
        }
    }

    return errors;
}

function highlightErrors(errors,tupel) {
    let errorindices = new Set;

    // Vorherige Fehler zurücksetzen
    document.querySelectorAll(".cell").forEach(cell => {
        cell.classList.remove(`error`);
    });

    document.addEventListener('DOMContentLoaded', () => {
        gridState = Array.from(document.querySelectorAll('#grid .cell'));
    });

    let size = tupel.size;
    errors.forEach(error => {
        switch (error[0]) {
            case "Z": { // Mehrere Zeilen
                const rows = error[1];
                rows.forEach(row => {
                    for (let col = 0; col < size; col++) {
                        const index = (row-1) * size + col;
                        errorindices.add(index);
                    }
                });
                break;
            }
            case "S": { // Mehrere Spalten
                const cols = error[1];
                cols.forEach(col => {
                    const c = col - 1;
                    for (let row = 0; row < size; row++) {
                        const index = row * size + c;
                        errorindices.add(index);
                    }
                });
                break;
            }
            case "E": { // Einzelnes Element
                errorindices.add(error[1]);
                errorindices.add(error[2]);
                break;
            }
        }
        applyErrors(errorindices);

    });
    const indicesDiv = document.getElementById("indices");
}

function applyErrors(indices){
    const cells = document.querySelectorAll("#grid .cell");
    indices.forEach(index => {
        const cell = cells[index];
        if (cell) {
            cell.classList.add("error"); // Markiere Fehler
        }
    });
}

/*
macht aus dem Hintergrundbild ein schwarz weiß Bild, damit im PDF später der Hintergrund auch schwarz weiß ist
 */
function convertImageToGrayscale(src) {
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

/*Zeichnet das partielle Sudoku als Vektorgraphik
macht die markierten Felder grau
unterscheidet zwischen voll und partiell
Aktuell noch nur auf 9*9
 */
async function renderSudoku(data, solution, svg) {
    while (svg.childNodes.length > 1) {
        svg.removeChild(svg.lastChild);
    }

    if(hasBackgroundImage){
        const defs = document.createElementNS("http://www.w3.org/2000/svg", "defs");
        const pattern = document.createElementNS("http://www.w3.org/2000/svg", "pattern");
        pattern.setAttribute("id", "bgImage");
        pattern.setAttribute("patternUnits", "userSpaceOnUse");
        pattern.setAttribute("width", 50 * currentSize);
        pattern.setAttribute("height", 50 * currentSize);

        const image = document.createElementNS("http://www.w3.org/2000/svg", "image");
        const grayDataURL = await convertImageToGrayscale(imageURL);
        image.setAttributeNS(null, "href", grayDataURL);
        image.setAttribute("width", 50 * currentSize);
        image.setAttribute("height", 50 * currentSize);
        image.setAttribute("preserveAspectRatio", "xMidYMid slice");

        pattern.appendChild(image);
        defs.appendChild(pattern);
        svg.appendChild(defs);
    }
    const rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');

    rect.setAttribute('x', 0);
    rect.setAttribute('y', 0);
    rect.setAttribute('width', 50 * currentSize);
    rect.setAttribute('height', 50 * currentSize);
    rect.setAttribute("fill", "white");
    svg.appendChild(rect);

    if (hasBackgroundImage) {
        const rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');

        rect.setAttribute('x', 0);
        rect.setAttribute('y', 0);
        rect.setAttribute('width', 50 * currentSize);
        rect.setAttribute('height', 50 * currentSize);
        rect.setAttribute("fill", "url(#bgImage)");
        svg.appendChild(rect);
    }

    for (let row = 0; row < currentSize; row++) {
        for (let col = 0; col < currentSize; col++) {
            let x = col * 50;
            let y = row * 50;
            let index = row * currentSize + col;

            const rect = document.createElementNS("http://www.w3.org/2000/svg", "rect");
            rect.setAttribute("x", x);
            rect.setAttribute("y", y);
            rect.setAttribute("width", 50);
            rect.setAttribute("height", 50);
            rect.setAttribute("class", "cell");
            rect.setAttribute("fill", "rgba(255, 255, 255, 0.5)");
            rect.setAttribute("stroke", "black");
            rect.setAttribute("stroke-width", "1");

            if (data[index] !== 0 && !hasBackgroundImage) {
                rect.setAttribute("fill", "rgba(255,255,255, 0.3)");
            }
            svg.appendChild(rect);


            if (data[index] !== 0 || svg === fullSvg) {
                const text = document.createElementNS("http://www.w3.org/2000/svg", "text");

                text.setAttribute("x", x + 25);
                text.setAttribute("y", y + 38);
                text.setAttribute("font-size", "40px");
                text.setAttribute("text-anchor", "middle");
                text.textContent = solution[index];
                text.setAttribute("fill", "black");
                svg.appendChild(text);


            }
        }
    }
    let lines = [];

    if (currentSize === 9) {
        lines = [
            {x1: 0, y1: 150, x2: 450, y2: 150},
            {x1: 0, y1: 300, x2: 450, y2: 300},
            {x1: 150, y1: 0, x2: 150, y2: 450},
            {x1: 300, y1: 0, x2: 300, y2: 450},
        ];
        svg.setAttribute('viewBox', `0 0 450 450`);
        svg.setAttribute('width', '450');
        svg.setAttribute('height', '450');
    } else if (currentSize === 6) {
        lines = [
            {x1: 0, y1: 100, x2: 300, y2: 100},
            {x1: 0, y1: 200, x2: 300, y2: 200},
            {x1: 150, y1: 0, x2: 150, y2: 300},
        ];
        svg.setAttribute('viewBox', `0 0 300 300`);
        svg.setAttribute('width', '300');
        svg.setAttribute('height', '300');
    } else {
        lines = [
            {x1: 0, y1: 100, x2: 200, y2: 100},
            {x1: 100, y1: 0, x2: 100, y2: 200},
        ];
        svg.setAttribute('viewBox', `0 0 200 200`);
        svg.setAttribute('width', '200');
        svg.setAttribute('height', '200');
    }

    lines.forEach(lineData => {
        const line = document.createElementNS("http://www.w3.org/2000/svg", "line");
        line.setAttribute("x1", lineData.x1);
        line.setAttribute("y1", lineData.y1);
        line.setAttribute("x2", lineData.x2);
        line.setAttribute("y2", lineData.y2);
        line.setAttribute("stroke", "black");
        line.setAttribute("stroke-width", "3");   // dickere Linien für Unterquadrate
        svg.appendChild(line);
    });

    const border = document.createElementNS("http://www.w3.org/2000/svg", "rect");
    border.setAttribute("x", 0);
    border.setAttribute("y", 0);
    border.setAttribute("width", 50 * currentSize);
    border.setAttribute("height", 50 * currentSize);
    border.setAttribute("fill", "none");
    border.setAttribute("stroke", "black");
    border.setAttribute("stroke-width", 8);   // noch dickerer Rahmen außen
    svg.appendChild(border);
}

/*
Funktion um das PDF zu downloaden
Mit svg kann man angeben ob das volle oder nur das partielle
 */
async function downloadSudoku(svg) {
    const pdf = new jsPDF({ unit: 'pt', format: [50*currentSize, 50*currentSize] });

    pdf.setFillColor(255, 255, 255);
    pdf.rect(0, 0, 50*currentSize, 50*currentSize, 'F');

    svg.style.display = "block";
    await svg2pdf(svg, pdf, { x: 0, y: 0, scale: 1 });
    if(svg === fullSvg){
        svg.style.display = "none";
        pdf.save("solution.pdf");
        return;
    }
    pdf.save("sudoku.pdf");
}

function removealllisteners() {
    for (let i = 0; i < listofCells.length; i++) {
        listofCells[i][0].removeEventListener('mouseover', markingModeFunction);
        listofCells[i][0].removeEventListener('mousedown', markingModeFunctionSingleClick);
    }
}

function insertNumberssubfunction(e, index) {
    document.querySelectorAll(".cell").forEach(cell => {
        cell.classList.remove("error");
    });
    const regex = new RegExp(`[^1-${currentSize}]`, 'g');
    const val = e.target.value.replace(regex, '').slice(0, 1);
    e.target.value = val;

    // Update state and changes (like in createGrid)
    gridState[index] = val && gridState[index] !== 0 ? parseInt(val, 10)+1 : 0;
}

function insertNumbers() {
    if (!insertNumberstrue) {
        removealllisteners();
        insertNumberstrue = true;
        for (let i = 0; i < listofCells.length; i++) {
            const cell = listofCells[i][0];
            const index = listofCells[i][1]
            const value = gridState[index]-1;
            cell.innerHTML = '';
            if(value >= 0) {
                // Create and add input, pre-filled
                const input = document.createElement('input');
                input.setAttribute('maxlength', '1');
                input.setAttribute('type', 'text');
                input.value = ((gridState[index] && gridState[index] >= 2) ? value : '');
                input.addEventListener('input', (e) => insertNumberssubfunction(e, index));
                cell.appendChild(input);
            }
        }
    } else {
        insertNumberstrue = false;
        removealllisteners();
        for (let i = 0; i < listofCells.length; i++) {
            const cell = listofCells[i][0];
            const index = listofCells[i][1];
            const value = gridState[index]-1;
            if(cell.firstChild) {
                cell.removeChild(cell.firstChild);
            }
            cell.addEventListener('mouseover', markingModeFunction);
            cell.addEventListener('mousedown', markingModeFunctionSingleClick);
            cell.innerHTML = ((gridState[index] && gridState[index] >= 2) ? value : '');
            cell.style.color = 'black';  // Force visible color
            cell.style.fontSize = '30px';  // Force size
            cell.style.opacity = '1';  // Ensure not transparent
            cell.style.visibility = 'visible';
        }
    }
}

createGrid(currentSize);
document.getElementById("numbersBtn").style.display = "none";

function showSudokuRules() {
    document.getElementById("rulesModal").style.display = "flex";
}

function closeRulesModal() {
    document.getElementById("rulesModal").style.display = "none";
}

document.getElementById("rulesModal").addEventListener("click", function(e) {
    if (e.target.id === "rulesModal") {
        closeRulesModal();
    }
});
