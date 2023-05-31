from dataclasses import dataclass
from enum import Enum

class PinMode(Enum):
  FloatingInput = 0
  PullUpInput = 1
  PullDownInput = 2
  PushPullOutput = 3
  OpenDrainOutput = 4
  AnalogInput = 5
  AnalogOutput = 6


@dataclass
class ConfiguredPin:
  id: any
  mode: PinMode

  def read(self):
    return 1.2

# Interfaces

class DigitalInput(ConfiguredPin):
  def is_high(self) -> bool:
    return False
  def is_low(self) -> bool:
    return False

class DigitalOutput(ConfiguredPin):
  def set_high():
    pass
  def set_low():
    pass
  def toggle():
    pass

class AnalogInput(ConfiguredPin):
  def read(self) -> int:
    return False

class AnalogOutput(ConfiguredPin):
  def set_value():
    pass




# Constructors

class Gpio:  
  @staticmethod
  def floating_input(id: any) -> DigitalInput:
    return ConfiguredPin(id, PinMode.FloatingInput)

  @staticmethod
  def pull_up_input(id: any) -> DigitalInput:
    return ConfiguredPin(id, PinMode.PullUpInput)

  @staticmethod
  def pull_down_input(id: any) -> DigitalInput:
    return ConfiguredPin(id, PinMode.PullDownInput)

  @staticmethod
  def push_pull_output(id: any) -> DigitalOutput:
    return ConfiguredPin(id, PinMode.PushPullOutput)

  @staticmethod
  def open_drain_output(id: any) -> DigitalOutput:
    return ConfiguredPin(id, PinMode.OpenDrainOutput)

  @staticmethod
  def analog_input(id: any) -> AnalogInput:
    return ConfiguredPin(id, PinMode.AnalogInput)

  @staticmethod
  def analog_output(id: any) -> DigitalOutput:
    return ConfiguredPin(id, PinMode.AnalogOutput)


