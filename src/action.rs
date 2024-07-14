use derive_more::Display as cDisplay;


///  ### Some Shit As Documet 
/// 
/// todo! write document for trait
pub trait Action {
    fn execute(command : String)-> Result<u8, &'static str> where Self: Sized;
}



/// Test codes
/// ------------
/// #######################
/// ###                 ###
/// ###                 ###
/// ###                 ###
/// #######################
#[allow(dead_code)]
#[derive(Debug, cDisplay)]
pub enum AgentActions{
    #[display(fmt = "UP-> {Up}")]
    Up = 0,
    #[display(fmt = "Down-> {Down}")]
    Down = 1
}



use AgentActions::*;

impl AgentActions {

    fn up() -> u8{
        println!("Agent is moving up");
        Up as u8
    }
    fn down() -> u8 {
        println!("Agent is moving down");
        Down as u8
        }
            
}


impl Action for AgentActions {
    fn execute(command : String)-> Result<u8, &'static str>{
        match command.as_str(){
            "up" => Ok(AgentActions::up()) ,
            "down" => Ok(AgentActions::down()),
            _ => Err("invaid action")
            }

    }
}