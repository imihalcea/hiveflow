#[macro_export]
macro_rules! _type_inference_hint_parallel {
    ($($task:tt)+) => {
        compile_error!(concat!(
            "❌ Impossible d'inférer le type d'entrée pour parallel!.\n",
            "👉 Veuillez spécifier explicitement le type comme ceci :\n",
            "   parallel!(T => ...)\n",
            "🔍 Tâches fournies : ",
            stringify!($($task)+)
        ));
    };
}

#[macro_export]
macro_rules! _type_inference_hint_sequential {
    ($($task:expr),+ $(,)?) => {
        compile_error!(concat!(
            "❌ Impossible d'inférer le type d'entrée pour sequential!.\n",
            "👉 Veuillez spécifier explicitement le type comme ceci :\n",
            "   sequential!(T => ...)\n",
            "🔍 Tâches fournies : ",
            stringify!($($task),+)
        ));
    };
}
