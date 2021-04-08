use genetic_algorithm::{
    crossover::UniformCrossover, mutation::GaussianMutation, selection::RouletteWheelSelection,
    GeneticAlgorithm,
};
use neural_network::{topology::LayerTopology, Network};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    let layer1 = LayerTopology { neurons: 3 };
    let layer2 = LayerTopology { neurons: 2 };
    let layer3 = LayerTopology { neurons: 1 };

    let ntwk = Network::random(&[layer1, layer2, layer3]);
    log::info!("{:?}", ntwk);

    let r = ntwk.propagate((&[1.0, 2.1, 3.3]).to_vec());
    log::info!("{:?}", r);

    let ga = GeneticAlgorithm::new(
        RouletteWheelSelection::new(),
        UniformCrossover::new(),
        GaussianMutation::new(0.5, 0.5),
    );
    log::info!("{:?}", ga);
}
