use std::collections::VecDeque;

    #[derive(Debug)]
    pub struct SymmetricMatrix<T> {
        size: usize,
        data: Vec<T>,
    }

    impl<T> SymmetricMatrix<T>
    where
        T: Default + Copy,
    {
        pub fn new(size: usize) -> Self {
            SymmetricMatrix {
                size,
                data: vec![T::default(); (size + 1) * size / 2],
            }
        }
        pub fn size(&self) -> usize {
            self.size
        }
        pub fn get(&self, row: usize, col: usize) -> T {
            let index = self.index_for(row, col);
            self.data[index]
        }
        pub fn set(&mut self, row: usize, col: usize, value: T) {
            let index = self.index_for(row, col);
            self.data[index] = value;
        }
        fn index_for(&self, row: usize, col: usize) -> usize {
            if col > row {
                col * (col + 1) / 2 + row
            } else {
                row * (row + 1) / 2 + col
            }
        }
    }

    #[derive(Debug)]
    #[allow(clippy::upper_case_acronyms)]
    pub struct DBSCAN<T> {
        eps: T,
        min_points: usize,
        clusters: Vec<Option<usize>>,
        visited: Vec<bool>,
        current_cluster: usize,
    }

    impl<T> DBSCAN<T>
    where
        T: Default + Copy + PartialOrd,
    {
        pub fn new(eps: T, min_points: usize) -> Self {
            DBSCAN {
                eps,
                min_points,
                clusters: Vec::new(),
                visited: Vec::new(),
                current_cluster: 0,
            }
        }

        pub fn perform_clustering(&mut self, matrix: &SymmetricMatrix<T>) -> &Vec<Option<usize>> {
            self.clusters = vec![None; matrix.size()];
            self.visited = vec![false; matrix.size()];
            self.current_cluster = 0;

            for point in 0..matrix.size() {
                if self.visited[point] {
                    continue;
                }
                self.visited[point] = true;
                let neighbors = self.region_query(matrix, point);
                if neighbors.len() >= self.min_points {
                    self.expand_cluster(matrix, point, neighbors);
                    self.current_cluster += 1;
                }
            }

            self.clusters.as_ref()
        }

        fn region_query(&self, matrix: &SymmetricMatrix<T>, point: usize) -> VecDeque<usize> {
            let mut neighbors = VecDeque::new();
            for other_point in 0..matrix.size() {
                let dist = matrix.get(point, other_point);
                if dist <= self.eps {
                    neighbors.push_back(other_point);
                }
            }
            neighbors
        }

        fn expand_cluster(
            &mut self,
            matrix: &SymmetricMatrix<T>,
            point: usize,
            mut neighbors: VecDeque<usize>,
        ) {
            self.clusters[point] = Some(self.current_cluster);

            while let Some(other_point) = neighbors.pop_front() {
                if !self.visited[other_point] {
                    self.visited[other_point] = true;
                    let mut other_neighbors = self.region_query(matrix, other_point);
                    if other_neighbors.len() >= self.min_points {
                        neighbors.append(&mut other_neighbors);
                    }
                }
                if self.clusters[other_point].is_none() {
                    self.clusters[other_point] = Some(self.current_cluster);
                }
            }
        }
    }