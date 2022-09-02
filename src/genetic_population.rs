use rand::Rng;

use crate::{
    can_crossover::CanCrossover, can_mutate::CanMutate, environment::Environment,
    predictor::Predictor,
};

pub struct Connection {
    pub innovation_number: usize, 
    in_node: usize, 
    out_node: usize, 
}

pub struct GeneticPopulation<T: CanCrossover + Predictor + CanMutate, E: Environment> {
    population: Vec<T>,
    environment: E,
    generation: u32,
    prev_best: Option<T>,
    existing_connections: Option<Vec<Connection>>
}

impl<T: CanCrossover + Predictor + CanMutate, E: Environment> GeneticPopulation<T, E> {
    pub fn new(population: Vec<T>, environment: E) -> Self {
        GeneticPopulation {
            population,
            environment,
            generation: 0,
            prev_best: None,
        }
    }

    pub fn get_prev_best(&self) -> Option<&T> {
        self.prev_best.as_ref()
    }

    pub fn advance_generation(&mut self) {
        // Evaluate the fitness of all members of the population
        let mut scores = self.environment.evaluate_predictors(&self.population.iter().collect::<Vec<_>>());

        // Normalize the scores to have a minimum of zero and a sum of one
        let min_fitness = scores.iter().copied().reduce(f64::min).unwrap();
        scores = scores
            .into_iter()
            .map(|score| score - min_fitness)
            .collect();
        let total_fitness: f64 = scores.iter().sum();
        if total_fitness == 0.0 {
            scores.fill(1.0 / self.population.len() as f64);
        }
        scores = scores
            .into_iter()
            .map(|score| score / total_fitness)
            .collect();

        // Produce a new population with parents from the previous population
        let mut new_population = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..self.population.len() {
            // Select two parents from the population based on their fitness
            // TODO refactor out code duplication
            let parent1 = {
                let fitness_threshold = rng.gen_range(0.0..1.0);

                let mut running_total = 0.0;
                let mut index = 0;
                loop {
                    assert!(index < self.population.len());

                    running_total += scores[index];

                    if running_total > fitness_threshold {
                        break;
                    }

                    index += 1;
                }

                &self.population[index]
            };
            let parent2 = {
                let fitness_threshold = rng.gen_range(0.0..1.0);

                let mut running_total = 0.0;
                let mut index = 0;
                loop {
                    assert!(index < self.population.len());

                    running_total += scores[index];

                    if running_total > fitness_threshold {
                        break;
                    }

                    index += 1;
                }

                &self.population[index]
            };

            // The two parents will now produce offspring
            // Please, give them some privacy...
            let mut offspring = parent1.crossover(parent2);

            // Mutate the offspring
            offspring.mutate();

            // Add the offspring to the next generation
            new_population.push(offspring);
        }

        // Save the best member of the old population
        let mut best_score = None;
        let old_pop = std::mem::take(&mut self.population);
        for (member, score) in old_pop.into_iter().zip(scores) {
            if best_score.is_none() || best_score.unwrap() < score {
                self.prev_best = Some(member);
                best_score = Some(score);
            }
        }

        // Begin the next generation
        self.population = new_population;
        self.generation += 1;
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }
}
