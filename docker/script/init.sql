CREATE TABLE ratings (
    id UUID PRIMARY KEY,
    merchant_id UUID NOT NULL,
    rating DOUBLE PRECISION NOT NULL
);