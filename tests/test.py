# from society import society
import numpy as np
from society import Agent, Env
from tools import timeit, memory





def callback(action):
    reward = 0
    if action == 1:
        reward += 0.55
    elif action == 0:
        reward += 0.45
    elif action == 2:
        reward -= 0.55
    else:
        reward += 0.5
    return reward


class Matrix(Env):
    def __init__(self, name, action_space , observation_space) -> None:
        # Call the superclass constructor
        super().__init__()
        # super().__init__(name, action_space, observation_space)

        pass

    def step(self, state: np.ndarray, action: int) -> tuple:
        # reward, done, info = super().step(state, action)
        reward = callback(action)
        return reward+1, True, {"some": 0}
    
    def reset(self):
        return super().reset()
    def close(self) -> None:
        return super().close()
                 







class Nemo(Agent):
    def __init__(self, name, actions, utility = 0):
        super().__init__()
        pass
    def policy(self, observ):
        p = observ / observ.sum()
        
        return np.random.choice(self.actions, )

    def Returns(self) :
        return super().Returns()




env1_ = Env(name="matrix",
            action_space= np.arange(10),
            observation_space= np.random.randint(1,10 , (10,100)) )

# print(env1_.observation_space)



env1 = Matrix(name="matrix",
           action_space= np.arange(10),
           observation_space = np.random.randint(1,10 , (10,100)) 
           )
agent = Nemo("nemo", np.arange(5) )




print(env1,
      agent
            
, sep="\n")


@memory
@timeit
def main():
    for episode in range(500):
        for step in env1.observation_space:
            action = agent.policy(step)
            reward, _done,info = env1.step(step, action)
            agent.utility += reward


main()
print(agent.utility)
