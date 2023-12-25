use std::io;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, PartialEq, Serialize)]
enum UnknownType{
    P,
    V,
    I,
    R,   
}
impl Copy for UnknownType {}


#[derive(Debug, Clone)]
struct Unknowns{
    unknown_1: (UnknownType, f64),
    unknown_2: (UnknownType, f64),
}

#[derive(Serialize)]
pub struct OperationResult{
    result: f64,
    constant: UnknownType,
}

#[derive(Deserialize)]
pub struct Operation{
    pub operation_type: String, 
    pub unknown_1: String, 
    pub unknown_2: String, 
    pub value_unknown_1: f64, 
    pub value_unknown_2: f64
}

impl Operation {

    pub fn new(self) -> Result<OperationResult, io::Error> {

        let operation = self.check_operation(&self.operation_type)?;

        let unknowns = self.check_unknowns(&self.unknown_1,  &self.unknown_2, self.value_unknown_1, self.value_unknown_2)?;

        let result: OperationResult = self.result_operation(operation, unknowns)?;

        Ok(result)
    }

    fn result_operation(&self, operation_type: UnknownType, unknowns: Unknowns)-> Result<OperationResult, io::Error>{
        
        let (unknown_1, unknown_value_1) = unknowns.unknown_1;
        let (unknown_2, unknown_value_2) = unknowns.unknown_2;

        

        let operation_invalid = io::Error::new(io::ErrorKind::Other, "Invalid operation.");
            
        match  operation_type {
            UnknownType::P =>{
                match  unknown_1 {
                    UnknownType::V => {
                        match unknown_2 {
                            UnknownType::I => {
                                Ok(OperationResult {result: unknown_value_1 * unknown_value_2, constant: UnknownType::P})
                            },
                            UnknownType::R => {
                                Ok(OperationResult{result: ( unknown_value_1.powf(2.0)) / unknown_value_2, constant: UnknownType::P})
                            }
                            _ => Err(operation_invalid)?
                        }
                    }
                    UnknownType::R =>{
                        match unknown_2 {
                            UnknownType::I =>{
                                Ok(OperationResult{result: unknown_value_1 * unknown_value_2.powf(2.0), constant: UnknownType::P})
                            }
                            _ => Err(operation_invalid)?
                        }
                    }
                    _ => Err(operation_invalid)?
                }
            }
            
            UnknownType::I =>{
                match unknown_1 {
                    UnknownType::V =>{
                        match unknown_2 {
                            UnknownType::R => { 
                                Ok(OperationResult{result: unknown_value_1 / unknown_value_2,  constant: UnknownType::I})
                            }
                            _ => Err(operation_invalid)?
                        }
                    }
                    UnknownType::P =>{
                        match unknown_2 {
                            UnknownType::V =>{
                                Ok(OperationResult{result: unknown_value_1 / unknown_value_2,  constant: UnknownType::I})
                            }
                            UnknownType::R =>{
                                let result = unknown_value_1 / unknown_value_2;
                                Ok(OperationResult{result: result.sqrt(),  constant: UnknownType::I})
                            }
                            _ => Err(operation_invalid)?
                        }
                    }
                    _ => Err(operation_invalid)?
                }
            }

            UnknownType::V =>{
                match unknown_1 {
                    UnknownType::P =>{
                        match unknown_2 {
                            UnknownType::I => {
                                Ok(OperationResult { result: unknown_value_1 / unknown_value_2, constant: UnknownType::V })
                            }
                            UnknownType::R => {
                                Ok(OperationResult { result: (unknown_value_1 / unknown_value_2).sqrt(), constant: UnknownType::V })
                            }
                            _ => Err(operation_invalid)?
                        }
                        
                    }
                    UnknownType::R =>{
                        match unknown_2 {
                            UnknownType::I => {
                                Ok(OperationResult { result: unknown_value_1 * unknown_value_2, constant: UnknownType::V })
                            }
                            _ => Err(operation_invalid)?
                        }
                    }
                    _ => Err(operation_invalid)?  
                }
            }

            UnknownType::R =>{
                match unknown_1 {
                    UnknownType::V =>{
                        match unknown_2 {
                            UnknownType::I => {
                                Ok(OperationResult { result: unknown_value_1 / unknown_value_2, constant: UnknownType::R })
                            }

                            UnknownType::P => {
                                Ok(OperationResult { result: unknown_value_1.powf(2.0) / unknown_value_2, constant: UnknownType::R })
                            }
                            _ => Err(operation_invalid)?
                        }
                    }
                    UnknownType::P => {
                         match unknown_2 {
                            UnknownType::I => {
                                Ok(OperationResult { result: unknown_value_1 / unknown_value_2.powf(2.0), constant: UnknownType::R })
                            }
                            _ => Err(operation_invalid)?
                        }
                    }
                    _ => Err(operation_invalid)?
                }
            }    
        }  
    }
    

    fn check_operation<'a>(&self, operation_type: &str) -> Result<UnknownType, io::Error> {

        match operation_type.to_uppercase().as_str() {
            "P" => Ok(UnknownType::P),
            "V" => Ok(UnknownType::V),
            "I" => Ok(UnknownType::I),
            "R" => Ok(UnknownType::R),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid operation type. Types of operation accepted: P, V, I, R"))?
        }
    }
    
    fn check_unknowns<'a>(&self, unknown_1: &str, unknown_2: &str, value_unknown_1: f64, value_unknown_2: f64 ) -> Result<Unknowns, io::Error> {
        
        if unknown_1 == unknown_2{
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Types of unknowns are equal"))?
        }
        
        if unknown_1 != "V" && unknown_1 != "R" && unknown_1 != "I" && unknown_1 != "P" {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid unknown_1 type. Types of unknown accepted: P, V, I, R"))?
        }

        if unknown_2 != "V" && unknown_2 != "R" && unknown_2 != "I" && unknown_2 != "P" {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid unknown_2 type. Types of unknown accepted: P, V, I, R"))?
        }
        
        println!("{:?}, {:?}, {}, {}", unknown_1, unknown_2, value_unknown_1, value_unknown_1);
             
        let invalid_unknow: io::Error  = io::Error::new(io::ErrorKind::InvalidInput, "Invalid operation type. Types of operation accepted: P, V, I, R");

        match  unknown_1 {
            "P" => {
                match  unknown_2{
                    "V" => Ok(Unknowns{unknown_1: (UnknownType::P, value_unknown_1), unknown_2:(UnknownType::V, value_unknown_2)}),
                    "I" => Ok(Unknowns{unknown_1: (UnknownType::P, value_unknown_1), unknown_2:(UnknownType::I, value_unknown_2)}),
                    "R" => Ok(Unknowns{unknown_1: (UnknownType::P, value_unknown_1), unknown_2:(UnknownType::R, value_unknown_2)}),
                    _ => Err(invalid_unknow)
                }
            }

            "V" => {
                match  unknown_2{
                    "P" => Ok(Unknowns{unknown_1: (UnknownType::V, value_unknown_1), unknown_2:(UnknownType::P, value_unknown_2)}),
                    "I" => Ok(Unknowns{unknown_1: (UnknownType::V, value_unknown_1), unknown_2:(UnknownType::I, value_unknown_2)}),
                    "R" => Ok(Unknowns{unknown_1: (UnknownType::V, value_unknown_1), unknown_2:(UnknownType::R, value_unknown_2)}),
                    _ => Err(invalid_unknow)
                }
            }

            "I" => {
                match  unknown_2{
                    "P" => Ok(Unknowns{unknown_1: (UnknownType::I, value_unknown_1), unknown_2:(UnknownType::P, value_unknown_2)}),
                    "V" => Ok(Unknowns{unknown_1: (UnknownType::I, value_unknown_1), unknown_2:(UnknownType::V, value_unknown_2)}),
                    "R" => Ok(Unknowns{unknown_1: (UnknownType::I, value_unknown_1), unknown_2:(UnknownType::R, value_unknown_2)}),
                    _ => Err(invalid_unknow)
                }
            }
            
            "R" => {
                match  unknown_2{
                    "P" => Ok(Unknowns{unknown_1: (UnknownType::R, value_unknown_1), unknown_2:(UnknownType::P, value_unknown_2)}),
                    "V" => Ok(Unknowns{unknown_1: (UnknownType::R, value_unknown_1), unknown_2:(UnknownType::V, value_unknown_2)}),
                    "I" => Ok(Unknowns{unknown_1: (UnknownType::R, value_unknown_1), unknown_2:(UnknownType::I, value_unknown_2)}),
                    _ => Err(invalid_unknow)
                }
            }
            _ => Err(invalid_unknow)
        }
    }      
}