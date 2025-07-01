from ast import Call
import inspect
from typing import Callable
def my_wrapper(func:Callable):
    def wrapper(*args,**kwargs):
        sig = inspect.signature(func)
        bound_args = sig.bind(*args,**kwargs)
        bound_args.apply_defaults()
        for param_name,param_value in bound_args.arguments.items():
            print(f"Function args name {param_name} and its value is {param_value}")
        print(f"My wrapper is running {func.__qualname__}")
        return func(*args,**kwargs)
    return wrapper

@my_wrapper
def do():
    print("my specifial function")
    return 6

@my_wrapper
def another_test_func(name:str, age:int):
    print(f'Your name is {name} and your age {age}')

smt = do()
print(smt)

print(another_test_func("Josh",18))