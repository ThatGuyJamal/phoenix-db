import json
import socket
from typing import Any, Dict, Union, Optional

# Define server connection settings
SERVER_HOST = '127.0.0.1'
SERVER_PORT = 8080


# Define the commands
def insert_command(key: str, value: Any) -> Dict[str, Any]:
    return {
        "name": "INSERT",
        "keys": [key],
        "values": [value]
    }


def lookup_command(key: str) -> Dict[str, Any]:
    return {
        "name": "LOOKUP",
        "keys": [key],
        "values": None
    }


def delete_command(key: str) -> Dict[str, Any]:
    return {
        "name": "DELETE",
        "keys": [key],
        "values": None
    }


# Define the NetResponse structure
def parse_net_response(response: str) -> Dict[str, Optional[Union[Any, str]]]:
    try:
        return json.loads(response)
    except json.JSONDecodeError:
        print("Failed to decode JSON response.")
        return {"value": None, "error": "Invalid response format"}


# Send command to server and get response
def send_command(s: socket.socket, command: Dict[str, Any]) -> None:
    try:
        # Serialize the command to JSON
        message = json.dumps(command)
        s.sendall(message.encode('utf-8'))

        # Receive the response from the server
        response = s.recv(1024).decode('utf-8')

        # Parse and print the response
        response_data = parse_net_response(response)
        print(f"Server Response: {response_data}")

    except Exception as err:
        print(f"An error occurred while sending command: {err}")


# Test the insert, lookup, and delete commands
if __name__ == "__main__":
    try:
        # Create a TCP connection to the server
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
            sock.connect((SERVER_HOST, SERVER_PORT))
            print(f"Connected to server at {SERVER_HOST}:{SERVER_PORT}")

            # Test inserting various types of data
            print("Testing INSERT commands with various data types...")

            # Insert string
            send_command(sock, insert_command("key_string", "string_value"))

            # Insert integer
            send_command(sock, insert_command("key_integer", 123))

            # Insert float
            send_command(sock, insert_command("key_float", 123.45))

            # Insert list
            send_command(sock, insert_command("key_list", [1, 2, 3, 4]))

            # Insert dictionary
            send_command(sock, insert_command("key_dict", {"a": 1, "b": 2}))

            # Test looking up the inserted keys
            print("\nTesting LOOKUP commands...")
            send_command(sock, lookup_command("key_string"))
            send_command(sock, lookup_command("key_integer"))
            send_command(sock, lookup_command("key_float"))
            send_command(sock, lookup_command("key_list"))
            send_command(sock, lookup_command("key_dict"))

            # Test deleting keys
            print("\nTesting DELETE commands...")
            send_command(sock, delete_command("key_string"))
            send_command(sock, delete_command("key_integer"))
            send_command(sock, delete_command("key_float"))
            send_command(sock, delete_command("key_list"))
            send_command(sock, delete_command("key_dict"))

            # Test looking up deleted keys
            print("\nTesting LOOKUP for deleted keys...")
            send_command(sock, lookup_command("key_string"))
            send_command(sock, lookup_command("key_integer"))
            send_command(sock, lookup_command("key_float"))
            send_command(sock, lookup_command("key_list"))
            send_command(sock, lookup_command("key_dict"))

    except ConnectionRefusedError:
        print(f"Error: Could not connect to server at {SERVER_HOST}:{SERVER_PORT}")
    except Exception as e:
        print(f"An error occurred: {e}")
