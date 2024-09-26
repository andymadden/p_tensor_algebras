mod algebra;


#[cfg(test)]
mod tests {
    use crate::algebra::Algebra;

    #[test]
    fn can_create_algebra() {
        let algebra = Algebra {
            p_tensor: &vec![
                vec![
                    vec![1, 2],
                    vec![3, 4]
                ],
                vec![
                    vec![1, 2],
                    vec![3, 4]
                ]
            ],
            dim: 2
        };
        assert_eq!(algebra.p_tensor[0][0][0], 1);
    }

    #[test]
    fn can_create_algebraic_vector() {
        let algebra = Algebra {
            p_tensor: &vec![
                vec![
                    vec![1, 0],
                    vec![0, 1]
                ],
                vec![
                    vec![0, 1],
                    vec![1, 1]
                ]
            ],
            dim: 2
        };
        let data = vec![1, 2];
        let vector = algebra.create_vector(data);

        assert_eq!(vector.algebra.p_tensor, algebra.p_tensor);
        assert_eq!(vector.data[0], 1);
        assert_eq!(vector.data[1], 2);
    }

    #[test]
    fn can_get_index() {
        let algebra = Algebra {
            p_tensor: &vec![
                vec![
                    vec![1, 0],
                    vec![0, 1]
                ],
                vec![
                    vec![0, 0],
                    vec![0, 0]
                ]
            ],
            dim: 2
        };
        let data = vec![1, 2];
        let vector = algebra.create_vector(data);
        assert_eq!(vector[0], 1);
        assert_eq!(vector[1], 2);
    }

    #[test]
    fn can_add_vectors() {
        let algebra = Algebra {
            p_tensor: &vec![
                vec![
                    vec![1, 0],
                    vec![0, 1]
                ],
                vec![
                    vec![0, 0],
                    vec![0, 0]
                ]
            ],
            dim: 2
        };
        let data_a = vec![1, 2];
        let vector_a = algebra.create_vector(data_a.clone());

        let data_b = vec![3, 4];
        let vector_b = algebra.create_vector(data_b.clone());

        let vector_c = vector_a.clone() + vector_b.clone();
        assert_eq!(vector_c[0], 4);
        assert_eq!(vector_c[1], 6);
    }

    #[test]
    fn can_add_assign_vectors() {
        let algebra = Algebra {
            p_tensor: &vec![
                vec![
                    vec![1, 0],
                    vec![0, 1]
                ],
                vec![
                    vec![0, 0],
                    vec![0, 0]
                ]
            ],
            dim: 2
        };
        let data_a = vec![1, 2];
        let mut vector_a = algebra.create_vector(data_a.clone());

        let data_b = vec![3, 4];
        let vector_b = algebra.create_vector(data_b.clone());

        vector_a += vector_b;
        assert_eq!(vector_a[0], 4);
        assert_eq!(vector_a[1], 6);
    }

    #[test]
    fn performs_complex_multiplication() {
        let algebra = Algebra {
            p_tensor: &vec![
                vec![
                    vec![1, 0],
                    vec![0, -1]
                ],
                vec![
                    vec![0, 1],
                    vec![1, 0]
                ]
            ],
            dim: 2
        };
        let data_a = vec![1, 2];
        let vector_a = algebra.create_vector(data_a.clone());

        let data_b = vec![3, 4];
        let vector_b = algebra.create_vector(data_b.clone());

        let vector_c = vector_a.clone()*vector_b.clone();

        assert_eq!(vector_c[0], vector_a[0]*vector_b[0] - vector_a[1]*vector_b[1]);
        assert_eq!(vector_c[1], vector_a[0]*vector_b[1] + vector_a[1]*vector_b[0]);
    }

    #[test]
    fn performs_split_complex_multiplication() {
        let algebra = Algebra {
            p_tensor: &vec![
                vec![
                    vec![1, 0],
                    vec![0, 1]
                ],
                vec![
                    vec![0, 1],
                    vec![1, 0]
                ]
            ],
            dim: 2
        };

        let data_a = vec![1, 2];
        let vector_a = algebra.create_vector(data_a.clone());

        let data_b = vec![3, 4];
        let vector_b = algebra.create_vector(data_b.clone());

        let vector_c = vector_a.clone()*vector_b.clone();

        assert_eq!(vector_c[0], vector_a[0]*vector_b[0] + vector_a[1]*vector_b[1]);
        assert_eq!(vector_c[1], vector_a[0]*vector_b[1] + vector_a[1]*vector_b[0]);
    }

    #[test]
    fn performs_dual_number_multiplication() {
        let algebra = Algebra {
            p_tensor: &vec![
                vec![
                    vec![1, 0],
                    vec![0, 0]
                ],
                vec![
                    vec![0, 1],
                    vec![1, 0]
                ]
            ],
            dim: 2
        };
        let data_a = vec![1, 2];
        let vector_a = algebra.create_vector(data_a.clone());

        let data_b = vec![3, 4];
        let vector_b = algebra.create_vector(data_b.clone());

        let vector_c = vector_a.clone()*vector_b.clone();

        assert_eq!(vector_c[0], vector_a[0]*vector_b[0]);
        assert_eq!(vector_c[1], vector_a[0]*vector_b[1] + vector_a[1]*vector_b[0]);
    }
}
