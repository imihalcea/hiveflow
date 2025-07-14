use hiveflow_core::core::task::Task;

#[derive(Debug)]
struct Add(i32);

#[derive(Debug)]
struct Mul(i32);

#[derive(Debug)]
struct Sum;

#[derive(Debug)]
struct Square;

#[derive(Debug)]
struct Flatten;

#[async_trait::async_trait]
impl Task<i32, i32> for Add {
    async fn run(&self, input: i32) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        Ok(input + self.0)
    }
}

#[async_trait::async_trait]
impl Task<i32, i32> for Mul {
    async fn run(&self, input: i32) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        Ok(input * self.0)
    }
}

#[async_trait::async_trait]
impl Task<Vec<i32>, i32> for Sum {
    async fn run(&self, input: Vec<i32>) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        Ok(input.iter().sum())
    }
}

#[async_trait::async_trait]
impl Task<i32, i32> for Square {
    async fn run(&self, input: i32) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        Ok(input * input)
    }
}

#[async_trait::async_trait]
impl Task<Vec<Vec<i32>>, Vec<i32>> for Flatten {
    async fn run(&self, input: Vec<Vec<i32>>) -> Result<Vec<i32>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(input.into_iter().flatten().collect())
    }
}

#[cfg(test)]
mod sequential_test {
    use crate::core::macros::{Add, Mul};
    use hiveflow_core::core::task::Task;
    use hiveflow_macros::sequential;

    #[tokio::test]
    async fn test_sequential_macro_explicit_type() {
        let pipeline = sequential!(
            i32 =>
            Add(1), Add(2), Mul(3)
        );
        let result: i32 = pipeline.run(0).await.unwrap();
        assert_eq!(result, 9);
    }
}

#[cfg(test)]
mod parallel_test {
    use crate::core::macros::{Add, Mul};
    use hiveflow_core::core::task::Task;
    use hiveflow_macros::parallel;

    #[tokio::test]
    async fn test_parallel_macro_explicit_type() {
        let pipeline = parallel!(
            i32 =>
            Add(2),
            Mul(3),
            Add(2));
        let result:Vec<i32> = pipeline.run(3).await.unwrap();
        assert_eq!(result, vec![5, 9, 5]);
    }
}

#[cfg(test)]
mod combine_sequential_and_parallel_test {
    use crate::core::macros::{Add, Mul, Sum};
    use hiveflow_core::core::task::Task;
    use hiveflow_macros::parallel;
    use hiveflow_macros::sequential;

    #[tokio::test]
    async fn test_seq_par_seq() {
        let pipeline = sequential!(
            i32 =>
            Add(1),
            Add(2),// i32 -> i32
            parallel!(i32 => Mul(3), Mul(4)),      // i32 -> Vec<i32>
            Sum                                     // Vec<i32> -> i32
        );

        let result = pipeline.run(2).await.unwrap();
        assert_eq!(result, 35);
    }
}

#[cfg(test)]
mod flow_test{
    use crate::core::macros::{Add, Mul, Sum, Square, Flatten};
    use hiveflow_core::core::task::Task;
    use hiveflow_macros::{flow, sequential, parallel};

    #[tokio::test]
    async fn test_flow_sequential_and_parallel() {
        let pipeline = flow! {
            i32
                => Add(1)
                => Add(2) // block sequential
                => [ Mul(3), Mul(4) ] // block parallel
                => Sum // block sequential
        };

        let result = pipeline.run(1).await.unwrap();
        // Étapes : 1 + 1 = 2 → +2 = 4 → [4*3, 4*4] = [12, 16] → sum = 28
        assert_eq!(result, 28);
    }


    #[tokio::test]
    async fn test_flow_parallel_to_sequential() {
        let pipeline = flow! {
            i32
                =>[ Add(2), Mul(3) ]
                => Sum
                => Add(1_i32)
        };

        let result = pipeline.run(2).await.unwrap();
        // [2+2, 2*3] = [4, 6] → sum = 10 → +1 = 11
        assert_eq!(result, 11);
    }

    #[tokio::test]
    async fn test_flow_sequential_to_parallel() {
        let pipeline = flow! {
            i32
                => Add(1)
                => Add(2)
                => [ Mul(3), Mul(4) ]
        };

        let result: Vec<i32> = pipeline.run(2).await.unwrap();
        // Étapes : 2 + 1 = 3 → +2 = 5 → [5*3, 5*4] = [15, 20]
        assert_eq!(result, vec![15, 20]);
    }

    #[tokio::test]
    async fn test_flow_multiple_parallel_and_flatten(){
        let pipeline = flow! {
            i32
                => Add(1)
                => [ [Add(2), Mul(3)], [Square, Mul(2)] ] // deux blocs parallèles imbriqués
                => Flatten
        };

        let result: Vec<i32> = pipeline.run(2).await.unwrap();
        // Étapes : 2 + 1 = 3 → [3+2, 3*3] = [5, 9] // [3^2, 3*2] = [5, 9, 9, 6]
        assert_eq!(result, vec![5, 9, 9, 6]);
    }

    #[tokio::test]
    async fn test_flow_parallel_nested_sequential_to_parallel() {
        let pipeline = flow! {
            i32
                => Add(1) // bloc séquentiel
                => [Add(2), Mul(3)] // bloc parallèle
                => Sum // bloc séquentiel
                => [Square, Mul(2)] // bloc parallèle après le bloc séquentiel
        };

        let result: Vec<i32> = pipeline.run(2).await.unwrap();
        assert_eq!(result, vec![196, 28]);
    }

    //to do later
    // #[tokio::test]
    // async fn named_steps_flow() {
    //     let pipeline = flow! {
    //         i32
    //         => step("start" => Add(1))
    //         => step("parallel" => [Mul(2), Mul(3)])
    //         => step("final" => Sum)
    // };
    // 
    //     let result = pipeline.run(2).await.unwrap();
    //     assert_eq!(result, 28); // Étapes : 2 + 1 = 3 → +2 = 5 → [5*3, 5*4] = [15, 20] → sum = 35
    // }
}