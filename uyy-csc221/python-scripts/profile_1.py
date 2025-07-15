from cmath import sqrt



def cal_roots(a:int,b:int,c:int):
    first_root = (-1*b + sqrt(abs((b^2)-(4*a*c)))) / 2*a 
    second_root = (-1*b - sqrt(abs((b^2)-(4*a*c))))/2*a
    return [first_root,second_root]


a = int(input('Enter Value for A: '))
b = int(input("Enter Value for B: "))
c =  int(input('Enter Value for C: '))
roots = cal_roots(a,b,c)

print(f'The roots are {roots[0]} & {roots[1]}')