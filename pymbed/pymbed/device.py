# def MCUDevice(obj, device_type):
#   print(obj)
#   print(device_type)
#   return obj

class MCUDeviceImpl:
    pass


class MCUDevice:
    def __init__(self, device_type: MCUDeviceImpl):
        self.device_type = device_type

    def __call__(self, cls):
        return cls