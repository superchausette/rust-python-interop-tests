print("From python: you're importing rust_test")

def i2a(a: int) -> str:
    return f'"{a}"'

def concat(a: int, b: int) -> str:
    return f'"{a}{b}"'