# from society import society
import numpy as np
from society import Agent, Env
from tools import timeit, memory





def callback(action):
    reward = 0
    match action :
        case 1:
            reward = np.array([0.55])
        case  0:
            reward = np.array([0.45])
        case  2:
            reward = np.array([0.55])
        case 3 :
            reward = np.array([-.90])
        case 4 :
            reward = np.array([.358])
        case 5:
            reward = np.array([.4])
    return reward


class Matrix(Env):
    def __init__(self, name, action_space , observation_space) -> None:
        self.action_space = action_space
        self.observation_space = observation_space
        super().__init__()
        pass

    def step(self, state: np.ndarray, action: int) -> tuple:
        reward = callback(action)
        return reward, False, {"some": 0}
    
    def reset(self):
        return super().reset()
    def close(self) -> None:
        return super().close()
                 







class Nemo(Agent):
    def __init__(self, name, action_size, utility = 0):
        self.utility = utility
        self.action_size = action_size
        super().__init__()
        pass
    def policy(self, observ):
        # p = observ / observ.sum()
        return np.random.choice(self.action_size, p =[.3 , .2, .1 , .2 , .2] )

    def returns(self) :
        return super().returns()




# env1_ = Env(name="matrix",
#             action_space= np.arange(10),
#             observation_space= np.random.randint(1,10 , (10,100)) )

# print(env1_.observation_space)



env1 = Matrix(name="matrix",
           action_space= np.arange(10),
           observation_space = np.random.randint(1,10 , (1000,100,2) , dtype=np.int64) 
           )

agent_action =  np.arange(5)

agent_action.shape
# print("Z :", 
#     type(agent_action).__name__
# )

# print(env1.observation_space,agent_action.shape)

utility = np.zeros((
    env1.observation_space.shape[0],
    agent_action.shape[0]
))
agent = Nemo("nemo", action_size = agent_action, utility=utility )




print(env1,
      agent
            
, sep="\n")

def soft_max(table):
    new_table = np.zeros(table.shape)
    for idx , value in enumerate(table):
        new_table[idx] =  np.exp(value) / np.sum(np.exp(value))
    return new_table

@memory
@timeit
def main():
    for episode in range(500):
        for state in env1.observation_space:
            action = agent.policy(state)
            reward, _, _ = env1.step(state, action)
            agent.utility[state, action] += reward
    agent.utility = soft_max(agent.utility)
    return agent.utility

utility_ = main()


print(utility_ , utility_.shape)
