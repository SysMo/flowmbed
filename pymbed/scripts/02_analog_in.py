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
  analog_in = prf.Gpio.analog_input(id = "pc13")
  digital_in = prf.Gpio.pull_up_input(id = "pc14")
  
  @task.loop(each = 1000)
  def measure(self):
    analog_value = self.analog_in.read()
    digital_value = self.digital_in.is_high()
    self.serial.print(analog_value)
    self.serial.print(digital_value)

mcu1 = MCU1()
mcu1.measure()