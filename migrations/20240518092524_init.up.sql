-- Add up migration script here
CREATE TABLE IF NOT EXISTS Employee (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    position TEXT NOT NULL,
    department TEXT NOT NULL
                                    );

CREATE TABLE IF NOT EXISTS InventoryItem (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    serial_number TEXT,
    responsible_person_id INTEGER NOT NULL,
    material_account TEXT NOT NULL,
    name TEXT NOT NULL,
    count INTEGER,
    price REAL NOT NULL,
    type TEXT NOT NULL,
    CONSTRAINT fk_responsible_person FOREIGN KEY (responsible_person_id) REFERENCES Employee(id)
);

CREATE TABLE IF NOT EXISTS IssueRecord (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    employee_id INTEGER NOT NULL,
    inventory_item_id INTEGER NOT NULL,
    issue_date TEXT NOT NULL,
    return_date TEXT,
    status TEXT NOT NULL,
    comments TEXT,
    CONSTRAINT fk_employee FOREIGN KEY (employee_id) REFERENCES Employee(id),
    CONSTRAINT fk_inventory_item FOREIGN KEY (inventory_item_id) REFERENCES InventoryItem(id)
)