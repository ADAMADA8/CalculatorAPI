# CalculatorAPI

This project is a simple calculator API built purely for self-education purposes. It provides basic arithmetic operations such as addition, subtraction, multiplication, and division through RESTful API endpoints.
## API Endpoints

All endpoints are available under the prefix: `/api/v1/`

### Add
- **GET** `/api/v1/add?1,2,3`
  - **Description:** Adds all numbers in the comma-separated list.
  - **Example:** `/api/v1/add?1,2,3` → `6`

### Subtract
- **GET** `/api/v1/subtract?10,2,3`
  - **Description:** Subtracts all subsequent numbers from the first.
  - **Example:** `/api/v1/subtract?10,2,3` → `5`

### Multiply
- **GET** `/api/v1/multiply?2,3,4`
  - **Description:** Multiplies all numbers in the comma-separated list.
  - **Example:** `/api/v1/multiply?2,3,4` → `24`

### Divide
- **GET** `/api/v1/divide?100,2,5`
  - **Description:** Divides the first number by each subsequent number in order.
  - **Example:** `/api/v1/divide?100,2,5` → `10`
