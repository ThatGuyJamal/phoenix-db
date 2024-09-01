import json
import socket
from typing import Any

# Define server connection settings
SERVER_HOST = '127.0.0.1'
SERVER_PORT = 8080


# Define the commands
def insert_command(key: str, value: Any) -> dict:
    return {
        "name": "INSERT",
        "key": key,
        "value": value
    }


def lookup_command(key: str) -> dict:
    return {
        "name": "LOOKUP",
        "key": key,
        "value": None
    }


def delete_command(key: str) -> dict:
    return {
        "name": "DELETE",
        "key": key,
        "value": None
    }


# Define the NetResponse structure
def parse_net_response(response: str) -> dict:
    try:
        return json.loads(response)
    except json.JSONDecodeError:
        print("Failed to decode JSON response.")
        return {"value": None, "error": "Invalid response format"}


# Send command to server and get response
def send_command(command: dict) -> None:
    try:
        # Create a TCP connection to the server
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
            sock.connect((SERVER_HOST, SERVER_PORT))

            # Serialize the command to JSON
            message = json.dumps(command)
            sock.sendall(message.encode('utf-8'))

            # Receive the response from the server
            response = sock.recv(1024).decode('utf-8')

            # Parse and print the response
            response_data = parse_net_response(response)
            print(f"Server Response: {response_data}")

    except ConnectionRefusedError:
        print(f"Error: Could not connect to server at {SERVER_HOST}:{SERVER_PORT}")
    except Exception as e:
        print(f"An error occurred: {e}")


# Test the insert, lookup, and delete commands
if __name__ == "__main__":
    # Test inserting various types of data
    print("Testing INSERT commands with various data types...")

    # Insert string
    send_command(insert_command("key_string", "string_value"))

    # Insert integer
    send_command(insert_command("key_integer", 123))

    # Insert float
    send_command(insert_command("key_float", 123.45))

    # Insert list
    send_command(insert_command("key_list", [1, 2, 3, 4]))

    # Insert dictionary
    send_command(insert_command("key_dict", {"a": 1, "b": 2}))

    # Test looking up the inserted keys
    print("\nTesting LOOKUP commands...")
    send_command(lookup_command("key_string"))
    send_command(lookup_command("key_integer"))
    send_command(lookup_command("key_float"))
    send_command(lookup_command("key_list"))
    send_command(lookup_command("key_dict"))

    # Test deleting keys
    print("\nTesting DELETE commands...")
    send_command(delete_command("key_string"))
    send_command(delete_command("key_integer"))
    send_command(delete_command("key_float"))
    send_command(delete_command("key_list"))
    send_command(delete_command("key_dict"))

    # Test looking up deleted keys
    print("\nTesting LOOKUP for deleted keys...")
    send_command(lookup_command("key_string"))
    send_command(lookup_command("key_integer"))
    send_command(lookup_command("key_float"))
    send_command(lookup_command("key_list"))
    send_command(lookup_command("key_dict"))
