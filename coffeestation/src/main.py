# Libraries
import RPi.GPIO as GPIO
import time
import requests
import os

GPIO.cleanup()

# dashboard URL
SPYLENT_DASHBOARD_URL = os.getenv('DASHBOARD_URL', 'https://spylent.herokuapp.com/api/point')

# cleanup buffer
distance_buffer_idx = 0
distance_buffer_max = 5
distance_buffer = [0 for x in range(distance_buffer_max)]

# GPIO Mode (BOARD / BCM)
GPIO.setmode(GPIO.BCM)

# set GPIO Pins
GPIO_TRIGGER = 18
GPIO_ECHO = 24

# set GPIO direction (IN / OUT)
GPIO.setup(GPIO_TRIGGER, GPIO.OUT)
GPIO.setup(GPIO_ECHO, GPIO.IN)


def send_to_spylent_dashboard(tag='', value=None):
    res = requests.post(SPYLENT_DASHBOARD_URL, json={'tag': tag, 'value': value})
    print(res)
    print(type(tag))
    print(type(value))
    print(res.text)


def distance():
    # set Trigger to HIGH
    GPIO.output(GPIO_TRIGGER, True)

    # set Trigger after 0.01ms to LOW
    time.sleep(0.00001)
    GPIO.output(GPIO_TRIGGER, False)

    StartTime = time.time()
    StopTime = time.time()
    ticks_left = 10000
    # save StartTime
    while GPIO.input(GPIO_ECHO) == 0 and ticks_left:
        time.sleep(0.0000001)
        StartTime = time.time()
        ticks_left -= 1

    ticks_left = 10000
    # save time of arrival
    while GPIO.input(GPIO_ECHO) == 1 and ticks_left:
        time.sleep(0.0000001)
        StopTime = time.time()
        ticks_left -= 1

    # time difference between start and arrival
    TimeElapsed = StopTime - StartTime
    # multiply with the sonic speed (34300 cm/s)
    # and divide by 2, because there and back
    dist = (TimeElapsed * 34300) / 2

    if 0 < dist < 1000:
        return dist

def median(arr):
    return sorted(arr)[int(len(arr) / 2)]

if __name__ == '__main__':
    while True:
        try:
            dist = distance() or distance()
            if dist:
                distance_buffer[distance_buffer_idx] = dist
                if distance_buffer_idx == distance_buffer_max - 1 or True:
                    average_distance = sum(distance_buffer) / len(distance_buffer)
                    print("%s - Average Measured Distance = %.1f cm" % (time.time(), average_distance))
                    print("distance buffer:", distance_buffer, "median:", median(distance_buffer))
                    send_to_spylent_dashboard('distance', median(distance_buffer))
                distance_buffer_idx = (distance_buffer_idx + 1) % distance_buffer_max
        except Exception as e:
            print(e)
        except Error as e:
            print(e)

        time.sleep(0.5)

GPIO.cleanup()