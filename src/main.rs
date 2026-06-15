// Declaramos los módulos que escribirán los compañeros
pub mod modelo;
pub mod recorrido;
pub mod ruta;
use petgraph::dot::{Dot, Config};
//use petgraph::dot::{Dot, Config};

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║     RED DE TRANSPORTE GLOBAL — GRAFOS EN RUST     ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();

    // Llamada al trabajo del Integrante 1
    let (grafo, nodos) = modelo::construir_grafo();

    println!("[ Nodos agregados al grafo ]");
    for (id, nombre) in nodos {
        println!("  {:?} → {}", id, nombre);
    }
    println!();

    println!("┌─────────────────────────────────────────┐");
    println!("│         RESUMEN DEL GRAFO                │");
    println!("├─────────────────────────────────────────┤");
    println!("│  Ciudades (nodos):   {:>3}                │", grafo.node_count());
    println!("│  Rutas (aristas):    {:>3}                │", grafo.edge_count());
    println!("└─────────────────────────────────────────┘");
    println!();

    // Ejecución de rutas de prueba (Modificado para usar índices directamente)
    // El 0 corresponde a Berlín (el primer nodo agregado)
    

    recorrido::ejecutar_bfs_nativo(&grafo, petgraph::graph::NodeIndex::new(0));
    
    println!("═══════════════════════════════════════════");
    println!("  RUTA CON MENOS ESCALAS: Toronto → Sydney");
    println!("═══════════════════════════════════════════");
    // El 4 es Toronto y el 3 es Sydney
    ruta::bfs_ruta(&grafo, petgraph::graph::NodeIndex::new(4), petgraph::graph::NodeIndex::new(3));
    println!();

    println!("═══════════════════════════════════════════");
    println!("  RUTA CON MENOS ESCALAS: Berlin → Sydney");
    println!("═══════════════════════════════════════════");
    // El 0 es Berlin y el 3 es Sydney
    ruta::bfs_ruta(&grafo, petgraph::graph::NodeIndex::new(0), petgraph::graph::NodeIndex::new(3));
    println!();

    let dot_output = Dot::with_config(&grafo, &[Config::EdgeNoLabel]);
    println!("{:?}", dot_output);
}

