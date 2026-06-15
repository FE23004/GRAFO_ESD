// Declaramos los módulos que escribirán los compañeros
pub mod modelo;
pub mod recorrido;
pub mod ruta;

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
