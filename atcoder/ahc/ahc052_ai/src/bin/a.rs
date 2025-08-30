#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*, time::Instant};

#[derive(Debug, Clone)]
struct Game {
    n: usize,                    // grid size (always 30)
    m: usize,                    // number of robots (always 10)
    k: usize,                    // number of buttons (always 10)
    robots: Vec<(usize, usize)>, // robot positions (i, j)
    v_walls: Vec<Vec<bool>>,     // vertical walls between (i,j) and (i,j+1)
    h_walls: Vec<Vec<bool>>,     // horizontal walls between (i,j) and (i+1,j)
    visited: Vec<Vec<bool>>,     // tracks which cells have been visited
}

impl Game {
    fn new(sc: &mut Scanner) -> Self {
        let n: usize = sc.cin();
        let m: usize = sc.cin();
        let k: usize = sc.cin();

        // Read robot initial positions
        let mut robots = Vec::new();
        for _ in 0..m {
            let i: usize = sc.cin();
            let j: usize = sc.cin();
            robots.push((i, j));
        }

        // Read vertical walls (between (i,j) and (i,j+1))
        let mut v_walls = vec![vec![false; n - 1]; n];
        for i in 0..n {
            let wall_str = sc.cin::<String>();
            for j in 0..n - 1 {
                v_walls[i][j] = wall_str.chars().nth(j).unwrap() == '1';
            }
        }

        // Read horizontal walls (between (i,j) and (i+1,j))
        let mut h_walls = vec![vec![false; n]; n - 1];
        for i in 0..n - 1 {
            let wall_str = sc.cin::<String>();
            for j in 0..n {
                h_walls[i][j] = wall_str.chars().nth(j).unwrap() == '1';
            }
        }

        // Initialize visited grid - mark initial robot positions as visited
        let mut visited = vec![vec![false; n]; n];
        for &(i, j) in &robots {
            visited[i][j] = true;
        }

        Game {
            n,
            m,
            k,
            robots,
            v_walls,
            h_walls,
            visited,
        }
    }

    // Check if there's a wall between two adjacent cells
    fn has_wall(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let (fi, fj) = from;
        let (ti, tj) = to;

        if ti == fi && tj == fj + 1 {
            // Moving right: check vertical wall
            self.v_walls[fi][fj]
        } else if ti == fi && tj + 1 == fj {
            // Moving left: check vertical wall
            self.v_walls[fi][tj]
        } else if ti == fi + 1 && tj == fj {
            // Moving down: check horizontal wall
            self.h_walls[fi][fj]
        } else if ti + 1 == fi && tj == fj {
            // Moving up: check horizontal wall
            self.h_walls[ti][tj]
        } else {
            false // not adjacent cells
        }
    }

    // Check if a position is within bounds
    fn in_bounds(&self, pos: (usize, usize)) -> bool {
        pos.0 < self.n && pos.1 < self.n
    }

    // Move a robot according to an action, return new position
    fn move_robot(&self, robot_pos: (usize, usize), action: char) -> (usize, usize) {
        let (i, j) = robot_pos;
        let new_pos = match action {
            'U' => {
                if i > 0 {
                    (i - 1, j)
                } else {
                    (i, j)
                }
            }
            'D' => (i + 1, j),
            'L' => {
                if j > 0 {
                    (i, j - 1)
                } else {
                    (i, j)
                }
            }
            'R' => (i, j + 1),
            'S' => (i, j), // Stay in place
            _ => (i, j),
        };

        // Check bounds and walls
        if !self.in_bounds(new_pos) || self.has_wall(robot_pos, new_pos) {
            robot_pos // Stay in original position if move is invalid
        } else {
            new_pos
        }
    }

    // Execute one turn with given button configuration
    fn execute_turn(&mut self, button_config: &[Vec<char>], button: usize) {
        // Update robot positions
        for i in 0..self.m {
            let action = button_config[button][i];
            self.robots[i] = self.move_robot(self.robots[i], action);
            // Mark new position as visited
            let (ri, rj) = self.robots[i];
            self.visited[ri][rj] = true;
        }
    }

    // Count unvisited cells
    fn count_unvisited(&self) -> usize {
        let mut count = 0;
        for i in 0..self.n {
            for j in 0..self.n {
                if !self.visited[i][j] {
                    count += 1;
                }
            }
        }
        count
    }

    // Calculate current score
    fn score(&self, operations: usize) -> usize {
        let unvisited = self.count_unvisited();
        if unvisited == 0 {
            3 * self.n * self.n - operations
        } else {
            self.n * self.n - unvisited
        }
    }
}

struct GreedySolver {
    game: Game,
    button_config: Vec<Vec<char>>,
    operations: Vec<usize>,
}

impl GreedySolver {
    fn new(game: Game) -> Self {
        let button_config = vec![vec!['S'; game.m]; game.k];
        let operations = Vec::new();

        GreedySolver {
            game,
            button_config,
            operations,
        }
    }

    fn generate_button_config(&mut self) {
        // Create specialized button configurations
        // Button 0: All robots move right
        for robot in 0..self.game.m {
            self.button_config[0][robot] = 'R';
        }
        // Button 1: All robots move left
        for robot in 0..self.game.m {
            self.button_config[1][robot] = 'L';
        }
        // Button 2: All robots move up
        for robot in 0..self.game.m {
            self.button_config[2][robot] = 'U';
        }
        // Button 3: All robots move down
        for robot in 0..self.game.m {
            self.button_config[3][robot] = 'D';
        }
        // Button 4: All robots stay
        for robot in 0..self.game.m {
            self.button_config[4][robot] = 'S';
        }

        // Buttons 5-9: Mixed patterns for flexibility
        let patterns = [
            ['R', 'L', 'U', 'D', 'S', 'R', 'L', 'U', 'D', 'S'], // Alternating pattern
            ['U', 'U', 'R', 'R', 'D', 'D', 'L', 'L', 'S', 'S'], // Paired movements
            ['R', 'R', 'R', 'L', 'L', 'L', 'U', 'U', 'D', 'D'], // Directional groups
            ['S', 'R', 'S', 'L', 'S', 'U', 'S', 'D', 'S', 'R'], // Selective movement
            ['D', 'R', 'U', 'L', 'D', 'R', 'U', 'L', 'D', 'R'], // Circular pattern
        ];

        for (button_idx, pattern) in patterns.iter().enumerate() {
            for robot in 0..self.game.m {
                self.button_config[5 + button_idx][robot] = pattern[robot];
            }
        }
    }

    fn manhattan_distance(&self, pos1: (usize, usize), pos2: (usize, usize)) -> usize {
        let (i1, j1) = pos1;
        let (i2, j2) = pos2;
        ((i1 as i32 - i2 as i32).abs() + (j1 as i32 - j2 as i32).abs()) as usize
    }

    fn find_nearest_unvisited(&self, robot_pos: (usize, usize)) -> Option<(usize, usize)> {
        let mut min_dist = usize::MAX;
        let mut nearest = None;

        for i in 0..self.game.n {
            for j in 0..self.game.n {
                if !self.game.visited[i][j] {
                    let dist = self.manhattan_distance(robot_pos, (i, j));
                    if dist < min_dist {
                        min_dist = dist;
                        nearest = Some((i, j));
                    }
                }
            }
        }

        nearest
    }

    fn evaluate_button(&self, button: usize) -> f64 {
        let mut total_score = 0.0;
        let mut new_visits = 0;

        // Simulate the button press
        for robot_idx in 0..self.game.m {
            let action = self.button_config[button][robot_idx];
            let current_pos = self.game.robots[robot_idx];
            let new_pos = self.game.move_robot(current_pos, action);

            // Count new cells that would be visited
            if !self.game.visited[new_pos.0][new_pos.1] {
                new_visits += 1;
            }

            // Find nearest unvisited cell and calculate improvement
            if let Some(target) = self.find_nearest_unvisited(current_pos) {
                let current_dist = self.manhattan_distance(current_pos, target);
                let new_dist = self.manhattan_distance(new_pos, target);

                // Reward moves that get closer to unvisited cells
                if new_dist < current_dist {
                    total_score += (current_dist - new_dist) as f64;
                } else if new_dist > current_dist {
                    total_score -= (new_dist - current_dist) as f64 * 0.5;
                }
            }
        }

        // Heavily weight new cell visits
        total_score += new_visits as f64 * 10.0;
        total_score
    }

    fn choose_best_button(&self) -> usize {
        let mut best_button = 0;
        let mut best_score = f64::NEG_INFINITY;

        for button in 0..self.game.k {
            let score = self.evaluate_button(button);
            if score > best_score {
                best_score = score;
                best_button = button;
            }
        }

        best_button
    }

    fn solve(&mut self, deadline: Instant) {
        // Generate button configuration
        self.generate_button_config();

        let max_operations = 2 * self.game.n * self.game.n;

        while self.operations.len() < max_operations {
            if Instant::now() >= deadline {
                break;
            }

            // Choose the best button based on greedy evaluation
            let button = self.choose_best_button();

            // Execute the operation
            self.game.execute_turn(&self.button_config, button);
            self.operations.push(button);

            // Stop early if all cells are visited
            if self.game.count_unvisited() == 0 {
                break;
            }
        }

        trace!(self.operations.len(), self.game.count_unvisited());
    }

    fn output_solution(&self) {
        // Output button configuration
        for button in 0..self.game.k {
            for robot in 0..self.game.m {
                if robot > 0 {
                    print!(" ");
                }
                print!("{}", self.button_config[button][robot]);
            }
            println!();
        }

        // Output operations
        for &op in &self.operations {
            println!("{}", op);
        }
    }
}

#[derive(Clone)]
struct BeamState {
    game: Game,
    operations: Vec<usize>,
    score: f64,
}

impl BeamState {
    fn new(game: Game) -> Self {
        BeamState {
            score: 0.0,
            game,
            operations: Vec::new(),
        }
    }

    fn evaluate(&self, solver: &BeamSearchSolver) -> f64 {
        let unvisited = self.game.count_unvisited();

        // Primary objective: minimize unvisited cells
        let mut total_score = if unvisited == 0 {
            // Perfect coverage - maximize remaining operations
            10000.0 - self.operations.len() as f64
        } else {
            // Penalize unvisited cells heavily
            -(unvisited as f64 * 100.0)
        };

        // Secondary: minimize total distance to unvisited cells
        let mut total_distance = 0.0;
        for robot_idx in 0..self.game.m {
            let robot_pos = self.game.robots[robot_idx];
            if let Some(target) = solver.find_nearest_unvisited_for_eval(&self.game, robot_pos) {
                let dist = solver.manhattan_distance(robot_pos, target);
                total_distance += dist as f64;
            }
        }

        // Only care about distance if we haven't achieved perfect coverage
        if unvisited > 0 {
            total_score -= total_distance * 0.1;
        }

        total_score
    }
}

struct BeamSearchSolver {
    game: Game,
    button_configs: Vec<Vec<Vec<char>>>, // Multiple button configurations
    beam_width: usize,
    search_depth: usize,
    operations: Vec<usize>,
    best_config_idx: usize,
}

impl BeamSearchSolver {
    fn new(game: Game) -> Self {
        BeamSearchSolver {
            game,
            button_configs: Vec::new(),
            beam_width: 5,
            search_depth: 3,
            operations: Vec::new(),
            best_config_idx: 0,
        }
    }

    fn generate_multiple_button_configs(&mut self) {
        self.button_configs.clear();

        // Pattern 3 FIRST: Spiral/circular patterns (was the best performing)
        let mut config3 = vec![vec!['S'; self.game.m]; self.game.k];
        for robot in 0..self.game.m {
            config3[0][robot] = 'R';
            config3[1][robot] = 'L';
            config3[2][robot] = 'U';
            config3[3][robot] = 'D';
            config3[4][robot] = 'S';
        }
        let patterns3 = [
            ['R', 'D', 'L', 'U', 'R', 'D', 'L', 'U', 'R', 'D'],
            ['U', 'L', 'D', 'R', 'U', 'L', 'D', 'R', 'U', 'L'],
            ['D', 'R', 'U', 'L', 'D', 'R', 'U', 'L', 'D', 'R'],
            ['L', 'D', 'R', 'U', 'L', 'D', 'R', 'U', 'L', 'D'],
            ['S', 'R', 'D', 'L', 'U', 'S', 'R', 'D', 'L', 'U'],
        ];
        for (button_idx, pattern) in patterns3.iter().enumerate() {
            for robot in 0..self.game.m {
                config3[5 + button_idx][robot] = pattern[robot];
            }
        }
        self.button_configs.push(config3);

        // Pattern 1: Basic directional patterns
        let mut config1 = vec![vec!['S'; self.game.m]; self.game.k];
        // Basic directional buttons
        for robot in 0..self.game.m {
            config1[0][robot] = 'R';
            config1[1][robot] = 'L';
            config1[2][robot] = 'U';
            config1[3][robot] = 'D';
            config1[4][robot] = 'S';
        }
        let patterns1 = [
            ['R', 'L', 'U', 'D', 'S', 'R', 'L', 'U', 'D', 'S'],
            ['U', 'U', 'R', 'R', 'D', 'D', 'L', 'L', 'S', 'S'],
            ['R', 'R', 'R', 'L', 'L', 'L', 'U', 'U', 'D', 'D'],
            ['S', 'R', 'S', 'L', 'S', 'U', 'S', 'D', 'S', 'R'],
            ['D', 'R', 'U', 'L', 'D', 'R', 'U', 'L', 'D', 'R'],
        ];
        for (button_idx, pattern) in patterns1.iter().enumerate() {
            for robot in 0..self.game.m {
                config1[5 + button_idx][robot] = pattern[robot];
            }
        }
        self.button_configs.push(config1);

        // Pattern 2: Coverage-focused patterns
        let mut config2 = vec![vec!['S'; self.game.m]; self.game.k];
        for robot in 0..self.game.m {
            config2[0][robot] = 'R';
            config2[1][robot] = 'L';
            config2[2][robot] = 'U';
            config2[3][robot] = 'D';
            config2[4][robot] = 'S';
        }
        let patterns2 = [
            ['R', 'R', 'L', 'L', 'U', 'U', 'D', 'D', 'S', 'S'],
            ['U', 'R', 'D', 'L', 'U', 'R', 'D', 'L', 'U', 'R'],
            ['L', 'U', 'R', 'D', 'L', 'U', 'R', 'D', 'L', 'U'],
            ['D', 'D', 'U', 'U', 'R', 'R', 'L', 'L', 'S', 'S'],
            ['S', 'U', 'S', 'D', 'S', 'L', 'S', 'R', 'S', 'U'],
        ];
        for (button_idx, pattern) in patterns2.iter().enumerate() {
            for robot in 0..self.game.m {
                config2[5 + button_idx][robot] = pattern[robot];
            }
        }
        self.button_configs.push(config2);

        // Pattern 4: Spread-out patterns
        let mut config4 = vec![vec!['S'; self.game.m]; self.game.k];
        for robot in 0..self.game.m {
            config4[0][robot] = 'R';
            config4[1][robot] = 'L';
            config4[2][robot] = 'U';
            config4[3][robot] = 'D';
            config4[4][robot] = 'S';
        }
        let patterns4 = [
            ['R', 'S', 'L', 'S', 'U', 'S', 'D', 'S', 'R', 'S'],
            ['U', 'D', 'U', 'D', 'L', 'R', 'L', 'R', 'S', 'S'],
            ['R', 'U', 'L', 'D', 'R', 'U', 'L', 'D', 'S', 'S'],
            ['D', 'L', 'U', 'R', 'D', 'L', 'U', 'R', 'S', 'S'],
            ['S', 'S', 'R', 'R', 'S', 'S', 'L', 'L', 'S', 'S'],
        ];
        for (button_idx, pattern) in patterns4.iter().enumerate() {
            for robot in 0..self.game.m {
                config4[5 + button_idx][robot] = pattern[robot];
            }
        }
        self.button_configs.push(config4);
    }

    fn expand_state(&self, state: &BeamState, config_idx: usize) -> Vec<BeamState> {
        let mut next_states = Vec::new();

        for button in 0..self.game.k {
            let mut new_state = state.clone();
            new_state
                .game
                .execute_turn(&self.button_configs[config_idx], button);
            new_state.operations.push(button);
            new_state.score = new_state.evaluate(self);
            next_states.push(new_state);
        }

        next_states
    }

    fn beam_search(&self, initial_state: BeamState, depth: usize, config_idx: usize) -> Vec<usize> {
        let mut current_beam = vec![initial_state];
        let mut best_operations = Vec::new();
        let mut best_score = f64::NEG_INFINITY;

        for _ in 0..depth {
            let mut next_beam = Vec::new();

            for state in &current_beam {
                let expanded = self.expand_state(state, config_idx);
                next_beam.extend(expanded);
            }

            // Sort by score and keep top beam_width states
            next_beam.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
            next_beam.truncate(self.beam_width);

            // Update best solution
            if let Some(best_state) = next_beam.first() {
                if best_state.score > best_score {
                    best_score = best_state.score;
                    best_operations = best_state.operations.clone();
                }
            }

            current_beam = next_beam;

            // Early termination if no valid states
            if current_beam.is_empty() {
                break;
            }
        }

        best_operations
    }

    fn solve(&mut self, deadline: Instant) {
        self.generate_multiple_button_configs();

        let max_operations = 2 * self.game.n * self.game.n;
        let total_configs = self.button_configs.len();

        let mut best_score = 0;
        let mut best_operations = Vec::new();
        let mut best_config_idx = 0;

        // Try each button configuration
        for (config_idx, _) in self.button_configs.iter().enumerate() {
            let mut game_copy = self.game.clone();
            let mut operations = Vec::new();

            // Calculate remaining time and configs
            let remaining_time = deadline.saturating_duration_since(Instant::now());
            let remaining_configs = total_configs - config_idx;
            let time_per_remaining_config = remaining_time / remaining_configs as u32;
            let config_deadline = Instant::now() + time_per_remaining_config;

            // Run solver with this configuration
            while operations.len() < max_operations && Instant::now() < config_deadline {
                let remaining_cells = game_copy.count_unvisited();

                if remaining_cells == 0 {
                    break;
                }

                let next_button = if remaining_cells > 50 {
                    self.choose_enhanced_greedy_button_for_config(&game_copy, config_idx)
                } else {
                    self.choose_beam_search_button_for_config(&game_copy, &operations, config_idx)
                };

                game_copy.execute_turn(&self.button_configs[config_idx], next_button);
                operations.push(next_button);

                if game_copy.count_unvisited() == 0 {
                    break;
                }
            }

            let final_score = game_copy.score(operations.len());
            eprintln!(
                "Config {}: Score = {}, Operations = {}",
                config_idx,
                final_score,
                operations.len()
            );

            if final_score > best_score {
                best_score = final_score;
                best_operations = operations;
                best_config_idx = config_idx;
            }
        }

        // Use the best configuration
        self.operations = best_operations;
        self.best_config_idx = best_config_idx;

        trace!(self.operations.len(), self.game.count_unvisited());
        eprintln!("BEST_CONFIG: {}", best_config_idx);
    }

    fn evaluate_button_for_config(&self, game: &Game, button: usize, config_idx: usize) -> f64 {
        let mut total_score = 0.0;
        let mut new_visits = 0;

        for robot_idx in 0..game.m {
            let action = self.button_configs[config_idx][button][robot_idx];
            let current_pos = game.robots[robot_idx];
            let new_pos = game.move_robot(current_pos, action);

            if !game.visited[new_pos.0][new_pos.1] {
                new_visits += 1;
            }

            if let Some(target) = self.find_nearest_unvisited_for_game(game, current_pos) {
                let current_dist = self.manhattan_distance(current_pos, target);
                let new_dist = self.manhattan_distance(new_pos, target);

                if new_dist < current_dist {
                    total_score += (current_dist - new_dist) as f64;
                } else if new_dist > current_dist {
                    total_score -= (new_dist - current_dist) as f64 * 0.5;
                }
            }
        }

        total_score += new_visits as f64 * 10.0;
        total_score
    }

    fn manhattan_distance(&self, pos1: (usize, usize), pos2: (usize, usize)) -> usize {
        let (i1, j1) = pos1;
        let (i2, j2) = pos2;
        ((i1 as i32 - i2 as i32).abs() + (j1 as i32 - j2 as i32).abs()) as usize
    }

    fn find_nearest_unvisited_for_game(
        &self,
        game: &Game,
        robot_pos: (usize, usize),
    ) -> Option<(usize, usize)> {
        let mut min_dist = usize::MAX;
        let mut nearest = None;

        for i in 0..game.n {
            for j in 0..game.n {
                if !game.visited[i][j] {
                    let dist = self.manhattan_distance(robot_pos, (i, j));
                    if dist < min_dist {
                        min_dist = dist;
                        nearest = Some((i, j));
                    }
                }
            }
        }

        nearest
    }

    fn find_nearest_unvisited_for_eval(
        &self,
        game: &Game,
        robot_pos: (usize, usize),
    ) -> Option<(usize, usize)> {
        let mut min_dist = usize::MAX;
        let mut nearest = None;

        for i in 0..game.n {
            for j in 0..game.n {
                if !game.visited[i][j] {
                    let dist = self.manhattan_distance(robot_pos, (i, j));
                    if dist < min_dist {
                        min_dist = dist;
                        nearest = Some((i, j));
                    }
                }
            }
        }

        nearest
    }

    fn choose_enhanced_greedy_button_for_config(&self, game: &Game, config_idx: usize) -> usize {
        let mut best_button = 0;
        let mut best_score = f64::NEG_INFINITY;

        for button in 0..game.k {
            let mut total_score = 0.0;
            let mut new_visits = 0;

            for robot_idx in 0..game.m {
                let action = self.button_configs[config_idx][button][robot_idx];
                let current_pos = game.robots[robot_idx];
                let new_pos = game.move_robot(current_pos, action);

                // Massive reward for new cell visits
                if !game.visited[new_pos.0][new_pos.1] {
                    new_visits += 1;
                }

                // Distance-based scoring
                if let Some(target) = self.find_nearest_unvisited_for_game(game, current_pos) {
                    let current_dist = self.manhattan_distance(current_pos, target);
                    let new_dist = self.manhattan_distance(new_pos, target);

                    if new_dist < current_dist {
                        total_score += (current_dist - new_dist) as f64 * 2.0;
                    } else if new_dist > current_dist {
                        total_score -= (new_dist - current_dist) as f64;
                    }
                }
            }

            // Primary factor: new visits
            total_score += new_visits as f64 * 50.0;

            if total_score > best_score {
                best_score = total_score;
                best_button = button;
            }
        }

        best_button
    }

    fn choose_beam_search_button_for_config(
        &self,
        game: &Game,
        operations: &[usize],
        config_idx: usize,
    ) -> usize {
        // Use beam search for precise endgame
        let current_state = BeamState {
            game: game.clone(),
            operations: operations.to_vec(),
            score: 0.0,
        };

        let search_ops = self.beam_search(current_state, self.search_depth, config_idx);

        if !search_ops.is_empty() && search_ops.len() > operations.len() {
            search_ops[operations.len()]
        } else {
            self.choose_enhanced_greedy_button_for_config(game, config_idx)
        }
    }

    fn output_solution(&self) {
        // Output button configuration using the best configuration
        for button in 0..self.game.k {
            for robot in 0..self.game.m {
                if robot > 0 {
                    print!(" ");
                }
                print!(
                    "{}",
                    self.button_configs[self.best_config_idx][button][robot]
                );
            }
            println!();
        }

        // Output operations
        for &op in &self.operations {
            println!("{}", op);
        }
    }
}

struct RandomWalkSolver {
    game: Game,
    button_config: Vec<Vec<char>>,
    operations: Vec<usize>,
    rng: XorShift,
}

impl RandomWalkSolver {
    fn new(game: Game) -> Self {
        let button_config = vec![vec!['S'; game.m]; game.k];
        let operations = Vec::new();
        let rng = XorShift::new();

        RandomWalkSolver {
            game,
            button_config,
            operations,
            rng,
        }
    }

    fn generate_random_button_config(&mut self) {
        let actions = ['U', 'D', 'L', 'R', 'S'];

        for button in 0..self.game.k {
            for robot in 0..self.game.m {
                let action_idx = self.rng.next() % actions.len();
                self.button_config[button][robot] = actions[action_idx];
            }
        }
    }

    fn solve(&mut self, deadline: Instant) {
        // Generate random button configuration
        self.generate_random_button_config();

        // Keep performing random operations until time limit or max operations
        let max_operations = 2 * self.game.n * self.game.n;

        while self.operations.len() < max_operations {
            if Instant::now() >= deadline {
                break;
            }

            // Choose random button
            let button = self.rng.next() % self.game.k;

            // Execute the operation
            self.game.execute_turn(&self.button_config, button);
            self.operations.push(button);

            // Optional: stop early if all cells are visited
            if self.game.count_unvisited() == 0 {
                break;
            }
        }

        trace!(self.operations.len(), self.game.count_unvisited());
        let final_score = self.game.score(self.operations.len());
        trace!(final_score);

        // 評価システム用のスコア出力（リリースビルドでも出力）
        eprintln!("FINAL_SCORE: {}", final_score);
    }

    fn output_solution(&self) {
        // Output button configuration
        for button in 0..self.game.k {
            for robot in 0..self.game.m {
                if robot > 0 {
                    print!(" ");
                }
                print!("{}", self.button_config[button][robot]);
            }
            println!();
        }

        // Output operations
        for &op in &self.operations {
            println!("{}", op);
        }
    }
}

struct SingleRobotFocusSolver {
    game: Game,
    button_config: Vec<Vec<char>>,
    operations: Vec<usize>,
    focus_robot: usize,
    rng: XorShift,
}

impl SingleRobotFocusSolver {
    fn new(game: Game) -> Self {
        let button_config = vec![vec!['S'; game.m]; game.k];
        let operations = Vec::new();
        let rng = XorShift::new();

        SingleRobotFocusSolver {
            game,
            button_config,
            operations,
            focus_robot: 0,
            rng,
        }
    }

    fn generate_button_config(&mut self) {
        let actions = ['U', 'D', 'L', 'R', 'S'];

        // Button 0: Focus robot moves right, others random
        for robot in 0..self.game.m {
            if robot == self.focus_robot {
                self.button_config[0][robot] = 'R';
            } else {
                let action_idx = self.rng.next() % actions.len();
                self.button_config[0][robot] = actions[action_idx];
            }
        }
        // Button 1: Focus robot moves left, others random
        for robot in 0..self.game.m {
            if robot == self.focus_robot {
                self.button_config[1][robot] = 'L';
            } else {
                let action_idx = self.rng.next() % actions.len();
                self.button_config[1][robot] = actions[action_idx];
            }
        }
        // Button 2: Focus robot moves up, others random
        for robot in 0..self.game.m {
            if robot == self.focus_robot {
                self.button_config[2][robot] = 'U';
            } else {
                let action_idx = self.rng.next() % actions.len();
                self.button_config[2][robot] = actions[action_idx];
            }
        }
        // Button 3: Focus robot moves down, others random
        for robot in 0..self.game.m {
            if robot == self.focus_robot {
                self.button_config[3][robot] = 'D';
            } else {
                let action_idx = self.rng.next() % actions.len();
                self.button_config[3][robot] = actions[action_idx];
            }
        }
        // Button 4: All robots stay
        for robot in 0..self.game.m {
            self.button_config[4][robot] = 'S';
        }

        // Buttons 5-9: Mixed patterns with random movements
        for button_idx in 5..self.game.k {
            for robot in 0..self.game.m {
                if robot == self.focus_robot {
                    // Focus robot gets a specific pattern for variety
                    let focus_actions = ['R', 'L', 'U', 'D', 'S'];
                    let action_idx = (button_idx - 5) % focus_actions.len();
                    self.button_config[button_idx][robot] = focus_actions[action_idx];
                } else {
                    // Other robots move randomly
                    let action_idx = self.rng.next() % actions.len();
                    self.button_config[button_idx][robot] = actions[action_idx];
                }
            }
        }
    }

    fn find_closest_unvisited_cell(&self) -> Option<(usize, usize)> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let focus_pos = self.game.robots[self.focus_robot];
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]; // R, L, D, U
        let mut dist = vec![vec![usize::MAX; self.game.n]; self.game.n];
        let mut heap = BinaryHeap::new();

        dist[focus_pos.0][focus_pos.1] = 0;
        heap.push(Reverse((0, focus_pos)));

        while let Some(Reverse((d, pos))) = heap.pop() {
            // If we found an unvisited cell, return it
            if !self.game.visited[pos.0][pos.1] && pos != focus_pos {
                return Some(pos);
            }

            if d > dist[pos.0][pos.1] {
                continue;
            }

            for &(di, dj) in &directions {
                let new_i = pos.0 as i32 + di;
                let new_j = pos.1 as i32 + dj;

                if new_i < 0 || new_j < 0 {
                    continue;
                }

                let new_pos = (new_i as usize, new_j as usize);

                if !self.game.in_bounds(new_pos) || self.game.has_wall(pos, new_pos) {
                    continue;
                }

                // Cost is 3 for visited cells, 1 for unvisited cells
                let move_cost = if self.game.visited[new_pos.0][new_pos.1] {
                    3
                } else {
                    1
                };
                let new_dist = d + move_cost;

                if new_dist < dist[new_pos.0][new_pos.1] {
                    dist[new_pos.0][new_pos.1] = new_dist;
                    heap.push(Reverse((new_dist, new_pos)));
                }
            }
        }

        None
    }

    fn dijkstra_path(&self, start: (usize, usize), target: (usize, usize)) -> Option<Vec<char>> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let directions = [('U', (-1, 0)), ('D', (1, 0)), ('L', (0, -1)), ('R', (0, 1))];
        let mut dist = vec![vec![usize::MAX; self.game.n]; self.game.n];
        let mut parent = vec![vec![None; self.game.n]; self.game.n];
        let mut heap = BinaryHeap::new();

        dist[start.0][start.1] = 0;
        heap.push(Reverse((0, start)));

        while let Some(Reverse((d, pos))) = heap.pop() {
            if pos == target {
                break;
            }

            if d > dist[pos.0][pos.1] {
                continue;
            }

            for &(action, (di, dj)) in &directions {
                let new_i = pos.0 as i32 + di;
                let new_j = pos.1 as i32 + dj;

                if new_i < 0 || new_j < 0 {
                    continue;
                }

                let new_pos = (new_i as usize, new_j as usize);

                if !self.game.in_bounds(new_pos) || self.game.has_wall(pos, new_pos) {
                    continue;
                }

                // Cost is 3 for visited cells, 1 for unvisited cells
                let move_cost = if self.game.visited[new_pos.0][new_pos.1] {
                    3
                } else {
                    1
                };
                let new_dist = d + move_cost;

                if new_dist < dist[new_pos.0][new_pos.1] {
                    dist[new_pos.0][new_pos.1] = new_dist;
                    parent[new_pos.0][new_pos.1] = Some((pos, action));
                    heap.push(Reverse((new_dist, new_pos)));
                }
            }
        }

        if dist[target.0][target.1] == usize::MAX {
            return None;
        }

        let mut path = Vec::new();
        let mut current = target;

        while let Some((prev_pos, action)) = parent[current.0][current.1] {
            path.push(action);
            current = prev_pos;
        }

        path.reverse();
        Some(path)
    }

    fn choose_button_towards_target(&self, target: (usize, usize)) -> usize {
        let focus_pos = self.game.robots[self.focus_robot];

        if focus_pos == target {
            return 4; // Stay if already at target
        }

        // Use Dijkstra to find optimal path
        if let Some(path) = self.dijkstra_path(focus_pos, target) {
            if !path.is_empty() {
                let next_action = path[0];
                return match next_action {
                    'R' => 0,
                    'L' => 1,
                    'U' => 2,
                    'D' => 3,
                    _ => 4,
                };
            }
        }

        // If no path found, stay
        4
    }

    fn solve(&mut self, deadline: Instant) {
        // Generate button configuration focusing on one robot
        self.generate_button_config();

        let max_operations = 2 * self.game.n * self.game.n;

        while self.operations.len() < max_operations {
            if Instant::now() >= deadline {
                break;
            }

            // Find closest unvisited cell
            if let Some(target) = self.find_closest_unvisited_cell() {
                let button = self.choose_button_towards_target(target);

                // Execute the operation
                self.game.execute_turn(&self.button_config, button);
                self.operations.push(button);
            } else {
                // All cells visited
                break;
            }

            // Stop early if all cells are visited
            if self.game.count_unvisited() == 0 {
                break;
            }
        }

        trace!(self.operations.len(), self.game.count_unvisited());
        let final_score = self.game.score(self.operations.len());
        trace!(final_score);

        eprintln!("FINAL_SCORE: {}", final_score);
    }

    fn output_solution(&self) {
        // Output button configuration
        for button in 0..self.game.k {
            for robot in 0..self.game.m {
                if robot > 0 {
                    print!(" ");
                }
                print!("{}", self.button_config[button][robot]);
            }
            println!();
        }

        // Output operations
        for &op in &self.operations {
            println!("{}", op);
        }
    }
}

// Simple XorShift random number generator
struct XorShift {
    state: u64,
}

impl XorShift {
    fn new() -> Self {
        XorShift {
            state: 88172645463325252,
        }
    }

    fn next(&mut self) -> usize {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        (self.state % (1 << 31)) as usize
    }
}

fn main() {
    let mut sc = Scanner::default();
    let game = Game::new(&mut sc);

    trace!(game.n, game.m, game.k);
    trace!(game.robots);
    trace!(game.count_unvisited());

    let start_time = Instant::now();
    let deadline = start_time + std::time::Duration::from_millis(1900);

    // Try SingleRobotFocusSolver first
    let mut single_robot_solver = SingleRobotFocusSolver::new(game.clone());
    single_robot_solver.solve(deadline);
    let single_robot_score = single_robot_solver
        .game
        .score(single_robot_solver.operations.len());
    eprintln!("SingleRobotFocusSolver score: {}", single_robot_score);

    // Try BeamSearchSolver with remaining time
    let mut beam_search_solver = BeamSearchSolver::new(game.clone());
    beam_search_solver.solve(deadline);
    let beam_search_score = beam_search_solver
        .game
        .score(beam_search_solver.operations.len());

    eprintln!("BeamSearchSolver score: {}", beam_search_score);

    // Output the best solution
    if single_robot_score >= beam_search_score {
        eprintln!("FINAL_SCORE: {}", single_robot_score);
        eprintln!("Using SingleRobotFocusSolver solution");
        single_robot_solver.output_solution();
    } else {
        eprintln!("FINAL_SCORE: {}", beam_search_score);
        eprintln!("Using BeamSearchSolver solution");
        beam_search_solver.output_solution();
    }
}

// {{{
use std::io::{self, Write};
use std::str::FromStr;
#[derive(Default)]
pub struct Scanner {
    buffer: VecDeque<String>,
}
impl Scanner {
    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = io::stdin().read_line(&mut line);
            self.buffer = line.split_whitespace().map(|w| String::from(w)).collect();
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn usize1(&mut self) -> usize {
        self.cin::<usize>() - 1
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.cin::<String>().chars().collect()
    }
    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
    pub fn pair<S: FromStr, T: FromStr>(&mut self) -> (S, T) {
        (self.cin::<S>(), self.cin::<T>())
    }
}
fn flush() {
    io::stdout().flush().unwrap();
}
#[macro_export]
macro_rules! min {
    (.. $x:expr) => {{
        let mut it = $x.iter();
        it.next().map(|z| it.fold(z, |x, y| min!(x, y)))
    }};
    ($x:expr) => ($x);
    ($x:expr, $($ys:expr),*) => {{
        let t = min!($($ys),*);
        if $x < t { $x } else { t }
    }}
}
#[macro_export]
macro_rules! max {
    (.. $x:expr) => {{
        let mut it = $x.iter();
        it.next().map(|z| it.fold(z, |x, y| max!(x, y)))
    }};
    ($x:expr) => ($x);
    ($x:expr, $($ys:expr),*) => {{
        let t = max!($($ys),*);
        if $x > t { $x } else { t }
    }}
}
#[macro_export]
macro_rules! clip {
    ($x:expr, $min:expr, $max:expr) => {{
        max!($min, min!($max, $x))
    }};
}
#[macro_export]
macro_rules! trace {
    (# $a:ident $(,)? $(;)? $($xs:expr),* $(,)? ) => {
        #[cfg(debug_assertions)]
        eprintln!("[{}] {} = {:?}", stringify!($a), stringify!($($xs),*), ($($xs),*))
    };
    ($($xs:expr),* $(,)?) => {
        #[cfg(debug_assertions)]
        eprintln!(">>> {} = {:?}", stringify!($($xs),*), ($($xs),*))
    };
}
#[macro_export]
macro_rules! comment {
    ($format:expr) => {
        eprintln!($format);
        println!($format);
    };
    ($format:expr, $($xs:expr),*) => {
        eprintln!($format, $($xs),*);
        println!($format, $($xs),*);
    }
}
#[macro_export]
macro_rules! put {
    (# $a:ident) => {println!("{}", stringify!($a))};
    (.. $x:expr) => {{
        let mut it = $x.iter();
        if let Some(x) = it.next() { print!("{}", x); }
        for x in it { print!(" {}", x); }
        println!("");
    }};
    ($x:expr) => { println!("{}", $x) };
    ($x:expr, $($xs:expr),*) => { print!("{} ", $x); put!($($xs),*) }
}
#[macro_export]
macro_rules! ndarray {
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}

// }}}
