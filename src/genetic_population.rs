use rand::Rng;

use crate::{
    can_crossover::CanCrossover, can_mutate::CanMutate, environment::Environment,
    predictor::Predictor,
};

pub struct GeneticPopulation<T: CanCrossover + Predictor + CanMutate, E: Environment> {
    population: Vec<T>,
    environment: E,
    generation: u32,
}

impl<T: CanCrossover + Predictor + CanMutate, E: Environment> GeneticPopulation<T, E> {
    pub fn new(population: Vec<T>, environment: E) -> Self {
        GeneticPopulation {
            population,
            environment,
            generation: 0,
        }
    }

    pub fn get_best(&mut self) -> &T {
        debug_assert!(self.population.len() > 0);

        // Evaluate the fitness of all members of the population
        let scores = self.environment.evaluate_predictors(&self.population);

        // Find the member of the population with the highest fitness
        let mut max_fitness = scores[0];
        let mut best = &self.population[0];
        for i in 1..self.population.len() {
            if scores[i] > max_fitness {
                max_fitness = scores[i];
                best = &self.population[i];
            }
        }

        best
    }

    pub fn advance_generation(&mut self) {
        // Evaluate the fitness of all members of the population
        let scores = self.environment.evaluate_predictors(&self.population);

        // Produce a new population with parents from the previous population
        let mut new_population = vec![];
        let total_fitness = scores.iter().sum();
        let mut rng = rand::thread_rng();
        for _ in 0..self.population.len() {
            // Select two parents from the population based on their fitness
            // TODO refactor out code duplication
            let parent1 = {
                let fitness_threshold = rng.gen_range(0.0..total_fitness);

                let mut running_total = 0.0;
                let mut index = 0;
                loop {
                    debug_assert!(index < self.population.len());

                    running_total += scores[index];

                    if running_total > fitness_threshold {
                        break;
                    }

                    index += 1;
                }

                &self.population[index]
            };
            let parent2 = {
                let fitness_threshold = rng.gen_range(0.0..total_fitness);

                let mut running_total = 0.0;
                let mut index = 0;
                loop {
                    debug_assert!(index < self.population.len());

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

        // Begin the next generation
        self.population = new_population;
        self.generation += 1;
    }
}
