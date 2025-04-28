use std::collections::HashSet;
use crate::apply_permutations::apply_permutations;
use crate::set_values::set_values;
use crate::sort::find_permutations;

/*
TODO !!!-------------------DIESE FUNKTION BENUTZEN BASTI!-------------------------!!! (LG Paul)
Funktion hat folgende Eingabeparameter: grid: das Feld mit 0 und 1 aus dem Frontend, grid_size: die Größe welche auch vom Frontend übergeben wurde
Funktion die Grid mit Nullen und Einsen nimmt,
die Zeilen und Spaltenvertauschungen berechnet und vornimmt
und anschließend die Stellen wo eine 1 stand mit Werten befüllt
Aktuell ist es so, dass bei 4x4 und 6x6 alle verschiedenen Befüllungen ausgegeben werden und bei 9x9 10000 zufällige
Die Befüllungen werden in einem Hash-Set ausgegeben mit den Zeilen- und Spaltenvertauschungen und dem Boolean ob gespiegelt wurde
Die Permutationen braucht man später noch um das grid dann wieder in die ursprüngliche Optik zu bringen
TODO Zeilen und Spaltenvertasuchung muss nach dem SAT wieder rückgängig gemacht werden
 */
pub fn fill_grid(grid: &[usize], grid_size: &usize) -> (HashSet<Vec<usize>>, Vec<usize>, Vec<usize>, bool) {
    let (row_permutation, col_permutation, mirror) = find_permutations(&grid, grid_size);
    let grid_permuted = apply_permutations(&grid, &row_permutation, &col_permutation, &mirror, grid_size);
    let results = set_values(&grid_permuted, grid_size);
    (results, row_permutation, col_permutation, mirror)
}