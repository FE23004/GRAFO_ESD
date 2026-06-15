use petgraph::visit::Bfs;
use crate::modelo::{RedAerea, NodeIdx};

/// Realiza una exploración por anchura nativa desde un nodo origen
pub fn ejecutar_bfs_nativo(grafo: &RedAerea, origen: NodeIdx) {
    println!("═══════════════════════════════════════════");
    println!("  RECORRIDO BFS DESDE BERLÍN");
    println!("  (orden en que se descubren las ciudades)");
    println!("═══════════════════════════════════════════");

    let mut bfs_iter = Bfs::new(grafo, origen);
    let mut paso = 1;

    while let Some(nodo_visitado) = bfs_iter.next(grafo) {
        println!("  Paso {:>2}: visitando → {}", paso, grafo[nodo_visitado]);
        paso += 1;
    }
    println!();
} 