CREATE TABLE IF NOT EXISTS benefit_claim (
    created_at timestamptz NOT NULL,
    body jsonb,
    id UUID NOT NULL PRIMARY KEY,
    person UUID NOT NULL REFERENCES person(id),
    received_at timestamptz NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_benefit_claim_person ON benefit_claim(person);

CREATE TABLE IF NOT EXISTS evaluation (
    benefit_claim UUID NOT NULL REFERENCES benefit_claim(id),
    body jsonb NOT NULL,
    created_at timestamptz NOT NULL,
    responded_at timestamptz NOT NULL,
    id UUID NOT NULL PRIMARY KEY
);
CREATE INDEX IF NOT EXISTS idx_evaluation_benefit_claim ON benefit_claim(id);