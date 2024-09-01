import json
import socket

# Define server connection settings
SERVER_HOST = '127.0.0.1'
SERVER_PORT = 8080


# Define the commands
def insert_command(key: str, value: str) -> dict:
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


# Test the insert and lookup commands
if __name__ == "__main__":
    # Test inserting a key-value pair
    print("Testing INSERT command...")
    send_command(insert_command("username", "john_doe"))

    # Test looking up a key
    print("\nTesting LOOKUP command...")
    send_command(lookup_command("username"))

    # Test looking up a non-existent key
    print("\nTesting LOOKUP for non-existent key...")
    send_command(lookup_command("non_existent_key"))
