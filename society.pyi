# society.pyi

from typing import List, Any
import numpy as np

class IDValueError(Exception):
    def __init__(self, msg: str) -> None: ...
    
class Agent:
    """Represents an agent with specific attributes."""
    name: str
    actions: np.ndarray | List[int] | np.ndarray 
    utility: List[int] | np.ndarray
    
    def __init__(self, 
                 name: str, 
                 actions: List[int] | np.ndarray,  
                 utility: List[int] | np.ndarray) -> None:
        """
        Initializes a new Agent.
        
        name: The name of the agent.
        actions: A list of action IDs.
        utility: The utility value of the agent.
        id: The unique ID of the agent.
        """
        ...
    def rule(self) -> Any:...
    def Returns(self) -> Any:...
    def __str__(self) -> str:...
    def __repr__(self) -> str:...
    def __eq__(self, other: Any) -> bool:...
    def __hash__(self) -> int:...
    
    @property
    def get_actions(self) -> np.ndarray:
        """
        Gets the actions as a NumPy array.
        
        :return: A NumPy array of action IDs.
        """
        ...
class Env:
    """Represents an agent with specific attributes."""
    id: int
    name: str
    action_space: np.ndarray | list[int]
    observation_space: np.ndarray
    
    def __init__(self, 
                name: str, 
                action_space: np.ndarray | list[int], 
                observation_space: np.ndarray,
                 ) -> None:
        """
        Initializes a new Agent.
        
        name: The name of the agent.
        actions: A list of action IDs.
        state_action: A list of lists representing state-action values.
        utility: The utility value of the agent.
        id: The unique ID of the agent.
        """
        ...
    @property
    def get_actions(self) -> np.ndarray:...
    def reset(self):...
    def step(self, action: int) -> tuple[np.ndarray, float, bool, dict]:...
    def close(self) -> None:...