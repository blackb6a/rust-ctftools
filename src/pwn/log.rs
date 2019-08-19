use log::Log;




pub struct LogImpl {

}
// impl Log for LogImpl {

// }
pub struct Logger<L: Log> {
    _logger: L
}

// impl<L: Log> Logger<L> {
//     fn __init__(logger: Option<L>) -> Self {
//         let _logger = match logger {
//             None => {
//                 logger_name = 
//             }
//             Some(logger) => logger,
//         };
//         Logger {
//             _logger
//         }
//     }
// }