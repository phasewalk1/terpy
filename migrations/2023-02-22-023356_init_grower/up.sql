CREATE TABLE terpenoid_screen (
  id SERIAL PRIMARY KEY,
  grower_id VARCHAR(255) NOT NULL,
  batch_id VARCHAR(255) NOT NULL,
  a_bisabolol REAL NOT NULL,
  a_humulene REAL NOT NULL,
  a_pinene REAL NOT NULL,
  a_terpinene REAL NOT NULL,
  b_caryophyllene REAL NOT NULL,
  b_myrcene REAL NOT NULL,
  b_pinene REAL NOT NULL,
  camphene REAL NOT NULL,
  caryophyllene_oxide REAL NOT NULL,
  delta_3_carene REAL NOT NULL,
  gamma_terpinene REAL NOT NULL,
  geraniol REAL NOT NULL,
  guaiol REAL NOT NULL,
  isopulegol REAL NOT NULL,
  linalool REAL NOT NULL,
  trans_nerolidol REAL NOT NULL,
  ocimene REAL NOT NULL,
  p_cymene REAL NOT NULL,
  eucalyptol REAL NOT NULL,
  terpinolene REAL NOT NULL
);
CREATE TABLE cannibanoid_screen (
  id SERIAL PRIMARY KEY,
  grower_id VARCHAR(255) NOT NULL,
  batch_id VARCHAR(255) NOT NULL,
  cbc REAL NOT NULL,
  cbd REAL NOT NULL,
  cbda REAL NOT NULL,
  cbdv REAL NOT NULL,
  cbg REAL NOT NULL,
  cbga REAL NOT NULL,
  cbn REAL NOT NULL,
  d9thc REAL NOT NULL,
  d8thc REAL NOT NULL,
  thcv REAL NOT NULL,
  thca REAL NOT NULL
);
CREATE TABLE test_results (
  id SERIAL PRIMARY KEY,
  grower_id VARCHAR(255) NOT NULL,
  batch_id VARCHAR(255) NOT NULL,
  cann INTEGER NOT NULL REFERENCES cannibanoid_screen(id),
  terp INTEGER NOT NULL REFERENCES terpenoid_screen(id)
);