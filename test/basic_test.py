import socket

def send_command(command):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect(('127.0.0.1', 8080))
        s.sendall(command.encode())
        response = s.recv(1024).decode()
        return response

def main():
    commands = [
        "INSERT <type>list</type> <key>my_list</key> <items><item>123</item><item>45.67</item><item>true</item><item>hello world!</item></items>",
        "LOOKUP my_list",

        "INSERT my_int_key 123 int",
        "INSERT my_float_key 456.78 float",
        "INSERT my_text_key 'Hello World' text",
        "INSERT my_bool_key true bool",
        'INSERT my_list 123,45.67,true,hello world list'
        "LOOKUP my_int_key",
        "LOOKUP my_float_key",
        "LOOKUP my_text_key",
        "LOOKUP my_bool_key"
    ]

    for cmd in commands:
        print(f"Sending: {cmd}")
        response = send_command(cmd)
        print(f"Response: {response}\n")

if __name__ == "__main__":
    main()
