/**
 * @class Sudoku
 * @description A base class to manage the core logic and state of a Sudoku grid.
 */
class Sudoku {
    /**
     * @param {number} size The length of one side of the Sudoku grid (e.g., 9 for a 9x9 grid).
     */
    constructor(size){
        /** @property {number} changes - Tracks the number of modifications made to the grid. */
        this.changes = 0;
        /** @property {number} size - The size of the grid (e.g., 9 for a 9x9 grid). */
        this.size = size;
        /** @property {number[]} gridState - A flat array representing the grid, initialized to zeros. */
        this.gridState = Array(size * size).fill(0);
    }

    /**
     * @description Resets the grid
     */
    reset(){
        this.gridState = Array(this.size * this.size).fill(0);
        this.changes = 0;
    }
    /**
     * @description Sets the value of a specific field in the grid.
     * @param {number} indexOfField - The index of the field to update in the flat grid array.
     * @param {number} value - The new value for the field (0 for empty, 1 for marked and 2-10 for marked with specified number i-1).
     * @throws {Error} If the field index or value is out of the valid range.
     */
    setField(indexOfField, value){
        if(indexOfField < 0 || (this.size * this.size) <= indexOfField){
            throw new Error("invalid field index");
        }
        else if(value < 0 || (this.size + 1) < value){
            throw new Error("invalid value");
        }
        if(this.gridState[indexOfField] === 0 && value !== 0){
            this.changes += 1;
        }
        else if(this.gridState[indexOfField] !== 0 && value === 0){
            this.changes -= 1;
        }
        this.gridState[indexOfField] = value;
    }

    /**
     * @description Returns the current grid state.
     * @returns {number[]} The currentGrid reference
     */
    getList() {
        return this.gridState;
    }

    /**
     * @description Returns the value at the specified place
     */
    getValueAt(index){
        return this.gridState[index];
    }

    /**
     * @description returns the current change count
     */
    getChangeCount(){
        return this.changes;
    }

    /**
     * @description Toggels the field, if non-zero it sets it to zero and if zero its sets it to one
     */
    toggleField(indexOfField){
        if(this.gridState[indexOfField] === 0) {
            this.gridState[indexOfField] = 1;
            this.changes += 1;
        }
        else {
            this.gridState[indexOfField] = 0;
            this.changes -= 1;

        }
    }
}

/**
 * @class Sudoku9
 * @extends Sudoku
 * @description Manages the logic for a standard 9x9 Sudoku grid.
 */
class Sudoku9 extends Sudoku {
    /**
     * @description Creates an instance of a 9x9 Sudoku.
     */
    constructor() {
        super(9);
    }

    /**
     * @description Gets all grid indices for a specific 3-row block.
     * @param {number} index - The index of the row block (0-2).
     * @returns {number[]} An array of grid indices within the specified row block.
     */
    getCellsInRowBlock(index){
        let cellList = [];
        for(let i=27*index; i<(27*index)+27; i++){
            cellList.push(i);
        }
        return cellList;
    }

    /**
     * @description Gets all grid indices for a specific 3-column block.
     * @param {number} index - The index of the column block (0-2).
     * @returns {number[]} An array of grid indices within the specified column block.
     */
    getCellsInColumnBlock(index){
        let cellList = [];
        for(let i=3*index; i<81; i+=9){
            cellList.push(i);
            cellList.push(i+1);
            cellList.push(i+2);
        }
        return cellList;
    }

    /**
     * @description Checks if a specific empty cell is a candidate for a hint. A cell is a candidate if its row or its column is not completely empty.
     * @param {number} index - The grid index of the cell to check.
     * @returns {boolean} True if the cell is a valid hint candidate, otherwise false.
     */
    checkCell(index){
        if(this.gridState[index] !== 0) {
            return false;
        }
        let cellRowIndex = Math.floor(index/9);
        let cellColIndex = index%9;
        let cellRow = this.getRow(cellRowIndex);
        let cellColumn = this.getColumn(cellColIndex);
        return (this.isEmpty(cellRow) || this.isEmpty(cellColumn));
    }

    /**
     * @description Generates a hint grid. Cells marked with '1' are recommended hint locations.
     * @returns {number[]} A new grid array where '1' indicates a suggested hint location.
     */
    getHintGrid() {
        let hintGrid = Array(81).fill(0);
        let colBlocksToCheck = this.checkColumnBlocks();
        let rowBlocksToCheck = this.checkRowBlocks();
        const cellsToCheck = new Set();

        for(const col of colBlocksToCheck){
            for(const cell of this.getCellsInColumnBlock(col)){
                cellsToCheck.add(cell);
            }
        }
        for(const row of rowBlocksToCheck){
            for(const cell of this.getCellsInRowBlock(row)){
                cellsToCheck.add(cell);
            }
        }

        for (let cell of cellsToCheck){
            if(this.checkCell(cell)){
                hintGrid[cell] = 1;
            }
        }
        return hintGrid;
    }

    /**
     * @description Checks the 3x3 column blocks to see if they require more hints.
     * @returns {number[]} An array of column block indices (0-2) that need more hints.
     */
    checkColumnBlocks() {
        let colHints = [];
        for(let i=0; i<3; i++){
            if (!this.checkBlock(this.getColumn(3*i), this.getColumn((3*i)+1), this.getColumn((3*i)+2) )) {
                colHints.push(i);
            }
        }
        return colHints;
    }

    /**
     * @description Checks the 3x3 row blocks to see if they require more hints.
     * @returns {number[]} An array of row block indices (0-2) that need more hints.
     */
    checkRowBlocks() {
        let rowHints = [];
        for(let i=0; i<3; i++){
            if (!this.checkBlock(this.getRow(3*i), this.getRow((3*i)+1), this.getRow((3*i)+2))) {
                rowHints.push(i);
            }
        }
        return rowHints;
    }

    /**
     * @description Checks if a block (composed of three rows or columns) has at least two non-empty lines.
     * @param {number[]} first - The first line (row or column) of the block.
     * @param {number[]} second - The second line (row or column) of the block.
     * @param {number[]} third - The third line (row or column) of the block.
     * @returns {boolean} True if the block has two or more non-empty lines, otherwise false.
     */
    checkBlock(first, second, third) {
        let notEmptyCount = 0;
        if (!this.isEmpty(first)) notEmptyCount++;
        if (!this.isEmpty(second)) notEmptyCount++;
        if (!this.isEmpty(third)) notEmptyCount++;
        return notEmptyCount >= 2;
    }

    /**
     * @description Gets all grid indices for a specific row.
     * @param {number} indexOfRow - The index of the row (0-8).
     * @returns {number[]} An array of grid indices that make up the specified row.
     */
    getRow(indexOfRow){
        let returnRow = []
        for(let i=0; i<9; i++){
            returnRow.push(9*indexOfRow+i);
        }
        return returnRow;
    }

    /**
     * @description Gets all grid indices for a specific column.
     * @param {number} indexOfColumn - The index of the column (0-8).
     * @returns {number[]} An array of grid indices that make up the specified column.
     */
    getColumn(indexOfColumn){
        let returnColumn = []
        for(let i=0; i<9; i++){
            returnColumn.push(indexOfColumn + (9*i));
        }
        return returnColumn;
    }

    /**
     * @description Checks if all fields corresponding to a list of indices are empty (value 0).
     * @param {number[]} listOfIndices - A list of indices to check in the grid state.
     * @returns {boolean} True if all corresponding fields are empty, otherwise false.
     */
    isEmpty(listOfIndices) {
        for(let i=0; i<listOfIndices.length; i++){
            if(this.gridState[listOfIndices[i]] !== 0){
                return false;
            }
        }
        return true;
    }

    /**
     * @description Logs the current 9x9 grid state to the console.
     */
    display() {
        for (let i=0; i<81; i+=9){
            console.log(this.gridState.slice(i, i + 9).join(' '));
        }
    }

    /**
     * @description Returns the indices of the fileds in the same block
     * @param {number} fieldIndex - The index of the field
     * @returns {number[]} The list of indices
     */
    getBlockIndices(fieldIndex){
        const colBlock = Math.floor((fieldIndex % 9)/3)
        const rowBlock = Math.floor(fieldIndex/27);
        let returnList = [];
        for(let i=0; i<3; i++){
            for(let j=0; j<3; j++){
                returnList.push((rowBlock*27)+(i)+(colBlock*3)+(9*j));
            }
        }
        return returnList;
    }
}

/**
 * @class Sudoku6
 * @extends Sudoku
 * @description Manages the logic for a 6x6 Sudoku grid (with 2x3 and 3x2 blocks).
 */
class Sudoku6 extends Sudoku {
    /**
     * @description Creates an instance of a 6x6 Sudoku.
     */
    constructor() {
        super(6);
    }

    /**
     * @description Checks if all fields corresponding to a list of indices are empty (value 0).
     * @param {number[]} listOfIndices - A list of indices to check in the grid state.
     * @returns {boolean} True if all corresponding fields are empty, otherwise false.
     */
    isEmpty(listOfIndices) {
        for (let i = 0; i < listOfIndices.length; i++) {
            if (this.gridState[listOfIndices[i]] !== 0) {
                return false;
            }
        }
        return true;
    }

    /**
     * @description Gets all grid indices for a specific 2-row block.
     * @param {number} index - The index of the row block (0-2).
     * @returns {number[]} An array of grid indices within the specified row block.
     */
    getCellsInRowBlock(index) {
        let cellList = [];
        const startRow = index * 2;
        for (let i = startRow * 6; i < (startRow + 2) * 6; i++) {
            cellList.push(i);
        }
        return cellList;
    }

    /**
     * @description Gets all grid indices for a specific 3-column block.
     * @param {number} index - The index of the column block (0-1).
     * @returns {number[]} An array of grid indices within the specified column block.
     */
    getCellsInColumnBlock(index) {
        let cellList = [];
        const startCol = index * 3;
        for (let row = 0; row < 6; row++) {
            for (let col = startCol; col < startCol + 3; col++) {
                cellList.push(row * 6 + col);
            }
        }
        return cellList;
    }

    /**
     * @description Checks if a specific empty cell is a candidate for a hint.
     * @param {number} index - The grid index of the cell to check.
     * @returns {boolean} True if the cell is a valid hint candidate, otherwise false.
     */
    checkCell(index) {
        if (this.gridState[index] !== 0) {
            return false;
        }
        const cellRowIndex = Math.floor(index / 6);
        const cellColIndex = index % 6;
        const cellRow = this.getRow(cellRowIndex);
        const cellColumn = this.getColumn(cellColIndex);
        return (this.isEmpty(cellRow) || this.isEmpty(cellColumn));
    }

    /**
     * @description Generates a hint grid for the 6x6 board.
     * @returns {number[]} A new grid array where '1' indicates a suggested hint location.
     */
    getHintGrid() {
        let hintGrid = Array(36).fill(0);
        const colBlocksToCheck = this.checkColumnBlocks();
        const rowBlocksToCheck = this.checkRowBlocks();
        const cellsToCheck = new Set();

        for (const col of colBlocksToCheck) {
            for (const cell of this.getCellsInColumnBlock(col)) {
                cellsToCheck.add(cell);
            }
        }
        for (const row of rowBlocksToCheck) {
            for (const cell of this.getCellsInRowBlock(row)) {
                cellsToCheck.add(cell);
            }
        }

        for (const cell of cellsToCheck) {
            if (this.checkCell(cell)) {
                hintGrid[cell] = 1;
            }
        }
        return hintGrid;
    }

    /**
     * @description Checks the two 3-column blocks to see if they require more hints.
     * @returns {number[]} An array of column block indices (0-1) that need more hints.
     */
    checkColumnBlocks() {
        const colHints = [];
        for (let i = 0; i < 2; i++) {
            if (!this.checkBlockOf3(this.getColumn(3 * i), this.getColumn(3 * i + 1), this.getColumn(3 * i + 2))) {
                colHints.push(i);
            }
        }
        return colHints;
    }

    /**
     * @description Checks the three 2-row blocks to see if they require more hints.
     * @returns {number[]} An array of row block indices (0-2) that need more hints.
     */
    checkRowBlocks() {
        const rowHints = [];
        for (let i = 0; i < 3; i++) {
            if (!this.checkBlockOf2(this.getRow(2 * i), this.getRow(2 * i + 1))) {
                rowHints.push(i);
            }
        }
        return rowHints;
    }

    /**
     * @description Checks if a block of 3 columns has at least two non-empty columns.
     * @param {number[]} first - The first column of the block.
     * @param {number[]} second - The second column of the block.
     * @param {number[]} third - The third column of the block.
     * @returns {boolean} True if the block has two or more non-empty columns.
     */
    checkBlockOf3(first, second, third) {
        let notEmptyCount = 0;
        if (!this.isEmpty(first)) notEmptyCount++;
        if (!this.isEmpty(second)) notEmptyCount++;
        if (!this.isEmpty(third)) notEmptyCount++;
        return notEmptyCount >= 2;
    }

    /**
     * @description Checks if a block of 2 rows has at least one non-empty row.
     * @param {number[]} first - The first row of the block.
     * @param {number[]} second - The second row of the block.
     * @returns {boolean} True if the block has one or more non-empty rows.
     */
    checkBlockOf2(first, second) {
        let notEmptyCount = 0;
        if (!this.isEmpty(first)) notEmptyCount++;
        if (!this.isEmpty(second)) notEmptyCount++;
        return notEmptyCount >= 1;
    }

    /**
     * @description Gets all grid indices for a specific row.
     * @param {number} indexOfRow - The index of the row (0-5).
     * @returns {number[]} An array of grid indices that make up the specified row.
     */
    getRow(indexOfRow) {
        const returnRow = [];
        for (let i = 0; i < 6; i++) {
            returnRow.push(6 * indexOfRow + i);
        }
        return returnRow;
    }

    /**
     * @description Gets all grid indices for a specific column.
     * @param {number} indexOfColumn - The index of the column (0-5).
     * @returns {number[]} An array of grid indices that make up the specified column.
     */
    getColumn(indexOfColumn) {
        const returnColumn = [];
        for (let i = 0; i < 6; i++) {
            returnColumn.push(indexOfColumn + 6 * i);
        }
        return returnColumn;
    }

    /**
     * @description Logs the current 6x6 grid state to the console.
     */
    display() {
        for (let i = 0; i < 36; i += 6) {
            console.log(this.gridState.slice(i, i + 6).join(' '));
        }
    }

    /**
     * @description Returns the indices of the fileds in the same block
     * @param {number} fieldIndex - The index of the field
     * @returns {number[]} The list of indices
     */
    getBlockIndices(fieldIndex) {
        const colBlock = Math.floor((fieldIndex % 6) / 3)
        const rowBlock = Math.floor(fieldIndex / 12);
        let returnList = [];
        for (let i = 0; i < 3; i++) {
            for (let j = 0; j < 2; j++) {
                returnList.push((rowBlock * 12) + (i) + (colBlock * 3) + (6 * j));
            }
        }
        return returnList;

    }
}
/**
 * @class Sudoku4
 * @extends Sudoku
 * @description Manages the logic for a 4x4 Sudoku grid (with 2x2 blocks).
 */
class Sudoku4 extends Sudoku {
    /**
     * @description Creates an instance of a 4x4 Sudoku.
     */
    constructor() {
        super(4);
    }

    /**
     * @description Checks if all fields corresponding to a list of indices are empty (value 0).
     * @param {number[]} listOfIndices - A list of indices to check in the grid state.
     * @returns {boolean} True if all corresponding fields are empty, otherwise false.
     */
    isEmpty(listOfIndices) {
        for (let i = 0; i < listOfIndices.length; i++) {
            if (this.gridState[listOfIndices[i]] !== 0) {
                return false;
            }
        }
        return true;
    }

    /**
     * @description Gets all grid indices for a specific 2-row block.
     * @param {number} index - The index of the row block (0-1).
     * @returns {number[]} An array of grid indices within the specified row block.
     */
    getCellsInRowBlock(index) {
        let cellList = [];
        const startRow = index * 2;
        for (let i = startRow * 4; i < (startRow + 2) * 4; i++) {
            cellList.push(i);
        }
        return cellList;
    }

    /**
     * @description Gets all grid indices for a specific 2-column block.
     * @param {number} index - The index of the column block (0-1).
     * @returns {number[]} An array of grid indices within the specified column block.
     */
    getCellsInColumnBlock(index) {
        let cellList = [];
        const startCol = index * 2;
        for (let row = 0; row < 4; row++) {
            for (let col = startCol; col < startCol + 2; col++) {
                cellList.push(row * 4 + col);
            }
        }
        return cellList;
    }

    /**
     * @description Checks if a specific empty cell is a candidate for a hint.
     * @param {number} index - The grid index of the cell to check.
     * @returns {boolean} True if the cell is a valid hint candidate, otherwise false.
     */
    checkCell(index) {
        if (this.gridState[index] !== 0) {
            return false;
        }
        const cellRowIndex = Math.floor(index / 4);
        const cellColIndex = index % 4;
        const cellRow = this.getRow(cellRowIndex);
        const cellColumn = this.getColumn(cellColIndex);
        return (this.isEmpty(cellRow) || this.isEmpty(cellColumn));
    }

    /**
     * @description Generates a hint grid for the 4x4 board.
     * @returns {number[]} A new grid array where '1' indicates a suggested hint location.
     */
    getHintGrid() {
        let hintGrid = Array(16).fill(0);
        const colBlocksToCheck = this.checkColumnBlocks();
        const rowBlocksToCheck = this.checkRowBlocks();
        const cellsToCheck = new Set();

        for (const col of colBlocksToCheck) {
            for (const cell of this.getCellsInColumnBlock(col)) {
                cellsToCheck.add(cell);
            }
        }
        for (const row of rowBlocksToCheck) {
            for (const cell of this.getCellsInRowBlock(row)) {
                cellsToCheck.add(cell);
            }
        }

        for (const cell of cellsToCheck) {
            if (this.checkCell(cell)) {
                hintGrid[cell] = 1;
            }
        }
        return hintGrid;
    }

    /**
     * @description Checks the two 2-column blocks to see if they require more hints.
     * @returns {number[]} An array of column block indices (0-1) that need more hints.
     */
    checkColumnBlocks() {
        const colHints = [];
        for (let i = 0; i < 2; i++) {
            if (!this.checkBlockOf2(this.getColumn(2 * i), this.getColumn(2 * i + 1))) {
                colHints.push(i);
            }
        }
        return colHints;
    }

    /**
     * @description Checks the two 2-row blocks to see if they require more hints.
     * @returns {number[]} An array of row block indices (0-1) that need more hints.
     */
    checkRowBlocks() {
        const rowHints = [];
        for (let i = 0; i < 2; i++) {
            if (!this.checkBlockOf2(this.getRow(2 * i), this.getRow(2 * i + 1))) {
                rowHints.push(i);
            }
        }
        return rowHints;
    }

    /**
     * @description Checks if a block of 2 rows or columns has at least one non-empty line.
     * @param {number[]} first - The first line (row or column) of the block.
     * @param {number[]} second - The second line (row or column) of the block.
     * @returns {boolean} True if the block has at least one non-empty line.
     */
    checkBlockOf2(first, second) {
        let notEmptyCount = 0;
        if (!this.isEmpty(first)) notEmptyCount++;
        if (!this.isEmpty(second)) notEmptyCount++;
        return notEmptyCount >= 1;
    }

    /**
     * @description Gets all grid indices for a specific row.
     * @param {number} indexOfRow - The index of the row (0-3).
     * @returns {number[]} An array of grid indices that make up the specified row.
     */
    getRow(indexOfRow) {
        const returnRow = [];
        for (let i = 0; i < 4; i++) {
            returnRow.push(4 * indexOfRow + i);
        }
        return returnRow;
    }

    /**
     * @description Gets all grid indices for a specific column.
     * @param {number} indexOfColumn - The index of the column (0-3).
     * @returns {number[]} An array of grid indices that make up the specified column.
     */
    getColumn(indexOfColumn) {
        const returnColumn = [];
        for (let i = 0; i < 4; i++) {
            returnColumn.push(indexOfColumn + 4 * i);
        }
        return returnColumn;
    }

    /**
     * @description Logs the current 4x4 grid state to the console.
     */
    display() {
        for (let i = 0; i < 16; i += 4) {
            console.log(this.gridState.slice(i, i + 4).join(' '));
        }
    }

    /**
     * @description Returns the indices of the fileds in the same block
     * @param {number} fieldIndex - The index of the field
     * @returns {number[]} The list of indices
     */
    getBlockIndices(fieldIndex) {
        const colBlock = Math.floor((fieldIndex % 4) / 2)
        const rowBlock = Math.floor(fieldIndex / 8);
        let returnList = [];
        for (let i = 0; i < 2; i++) {
            for (let j = 0; j < 2; j++) {
                returnList.push((rowBlock * 8) + (i) + (colBlock * 2) + (4 * j));
            }
        }
        return returnList;

    }
}