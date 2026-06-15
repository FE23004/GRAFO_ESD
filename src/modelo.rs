use petgraph::graph::UnGraph;

// Alias de tipo para que todo el grupo lo use de forma limpia
pub type RedAerea = UnGraph<String, u32>;
pub type NodeIdx = petgraph::graph::NodeIndex;

/// Inicializa el grafo con las ciudades y rutas aéreas del proyecto
pub fn construir_grafo() -> (RedAerea, Vec<(NodeIdx, &'static str)>) {
    let mut grafo = RedAerea::new_undirected();

    // Añadir nodos (ciudades)
    let berlin      = grafo.add_node("Berlin".to_string());
    let nueva_delhi = grafo.add_node("Nueva Delhi".to_string());
    let ciudad_mx   = grafo.add_node("Ciudad de Mexico".to_string());
    let sydney      = grafo.add_node("Sydney".to_string());
    let toronto     = grafo.add_node("Toronto".to_string());
    let nairobi     = grafo.add_node("Nairobi".to_string());

    // Guardamos una lista para que el Integrante 2 pueda imprimir el reporte de nodos
    let nodos_registrados = vec![
        (berlin, "Berlin"),
        (nueva_delhi, "Nueva Delhi"),
        (ciudad_mx, "Ciudad de Mexico"),
        (sydney, "Sydney"),
        (toronto, "Toronto"),
        (nairobi, "Nairobi"),
    ];

    // Añadir aristas (rutas)
    grafo.extend_with_edges(&[
        (berlin,      nueva_delhi, 5_836u32),
        (berlin,      ciudad_mx,   9_040u32),
        (berlin,      toronto,     6_336u32),
        (berlin,      nairobi,     6_353u32),
        (nueva_delhi, sydney,      9_982u32),
        (nueva_delhi, nairobi,     5_460u32),
        (ciudad_mx,   toronto,     3_240u32),
        (ciudad_mx,   sydney,     13_561u32),
        (toronto,     nairobi,    11_838u32),
        (nairobi,     sydney,     11_428u32),
    ]);

    (grafo, nodos_registrados)
}