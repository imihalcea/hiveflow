
#[macro_export]
macro_rules! sequential {
    ($a:expr, $b:expr) => {
        {
            move |input| async move {
                let r1 = $a.run(input).await?;
                $b.run(&r1).await
            }
        }
    };
    ($a:expr, $b:expr, $($rest:expr),+) => {
        {
            let composed = sequential!($a, $b);
            move |input| async move {
                let intermediate = composed(input).await?;
                sequential!($($rest),+)( &intermediate ).await
            }
        }
    };
}