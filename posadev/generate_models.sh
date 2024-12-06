#!/bin/bash

DERIVE_FLAGS="Clone, Debug, Identifiable, Queryable, QueryableByName, Insertable, Serialize, Deserialize"

echo "Running diesel migrations"
diesel migration run

echo "Gettting schema from database"
diesel print-schema > src/schema.rs

echo "Generating models"
diesel_ext -t -s src/schema.rs -m \
  -I "crate::schema::*"           \
  -I "rocket::serde::{ Deserialize, Serialize }"       \
  -d "${DERIVE_FLAGS}" > src/models.rs
