from society import society
import numpy as np
from society import Agent, Env

env1 = Env(name="matrix",action_space= np.arange(10),observation_space= np.random.randint(1,10 , (10,10)) , id = 1 , state = 12.)
print(env1)

ag1 = Agent( name = "neo0", 
                    actions = np.arange(10)  ,
                    utility = 0.1
                    )

ag2 = Agent( "smith", [1,2]  , 0.1)

# ag3 = society.Agent( "morphius", [1,2] )

class Nemo(Agent):
    def __init__(self, name, actions, utility):
        Agent.__init__(self)
        pass
    def rule(self, observ):
        Agent.rule(self)
        pass
        

ag3 = Nemo("nemo", [1,2], 0.0)
print( 
type(ag1.actions), 
type(ag2.actions), 
type(ag3.actions),
type(ag1.actions), type(ag1.actions) ,  
 type(ag1.utility), ag1.utility
,sep="\n")


for i in range(10):
    print(ag1.rule(env1))
