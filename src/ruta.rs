use std::collections::{HashMap, VecDeque};
use crate::modelo::{RedAerea, NodeIdx};

/// Encuentra e imprime la ruta con menos escalas entre dos puntos
pub fn bfs_ruta(grafo: &RedAerea, origen: NodeIdx, destino: NodeIdx) {
    let mut padre: HashMap<NodeIdx, NodeIdx> = HashMap::new();
    let mut cola: VecDeque<NodeIdx> = VecDeque::new();

    padre.insert(origen, origen);
    cola.push_back(origen);

    let mut encontrado = false;

    'bfs: while let Some(actual) = cola.pop_front() {
        for vecino in grafo.neighbors(actual) {
            if !padre.contains_key(&vecino) {
                padre.insert(vecino, actual);
                cola.push_back(vecino);

                if vecino == destino {
                    encontrado = true;
                    break 'bfs;
                }
            }
        }
    }

    if encontrado {
        let mut camino: Vec<String> = Vec::new();
        let mut nodo_actual = destino;

        loop {
            camino.push(grafo[nodo_actual].clone());
            let padre_del_actual = padre[&nodo_actual];
            if padre_del_actual == nodo_actual { break; }
            nodo_actual = padre_del_actual;
        }

        camino.reverse();
        let num_escalas = if camino.len() > 2 { camino.len() - 2 } else { 0 };

        println!("  Ruta encontrada con {} escala(s):", num_escalas);
        println!("  {}", camino.join(" → "));
    } else {
        println!("  No existe ruta entre {} y {}.", grafo[origen], grafo[destino]);
    }
}