#from pymbed import MCUDevice, MCUPeripheral
import sys
sys.path.append('.')

import pymbed
from pymbed import MCUDevice, task
from pymbed import peripherals as prf

# def example1():
  # '''Analog read'''
@MCUDevice(device_type = pymbed.devices.STM32F407)
class MCU1:
  serial = prf.Uart.serial_printer()
  led_pin = prf.Gpio.push_pull_output(id = "pc13")

  def init(self):
    self.serial.print("Init")
    self.led_pin.set_high()

  def idle(self):
    pass

  @task.loop(each = 1000)
  def blink(self):
    self.serial.print("Toggle")
    self.led_pin.toggle()



mcu1 = MCU1()
mcu1.build_and_flash()

# mcu1.blink()
# print(dir(mcu1))
# # example1()

# import ast, inspect
# print(ast.parse(inspect.getsource(mcu1.__class__)))



