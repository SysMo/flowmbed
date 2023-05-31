from dataclasses import dataclass

@dataclass
class Uart:
    speed: int = 9600

    @staticmethod
    def serial_printer():
        return Uart()
    
    def print(self, x: any):
        print(x)
        