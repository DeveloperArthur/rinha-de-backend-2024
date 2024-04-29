CREATE TABLE clientes (
	id SERIAL PRIMARY KEY,
	nome VARCHAR(50) NOT NULL,
	limite INTEGER NOT NULL,
    saldo INTEGER NOT NULL
);

CREATE TABLE transacoes (
	id SERIAL PRIMARY KEY,
	cliente_id INTEGER NOT NULL,
	valor INTEGER NOT NULL,
	tipo CHAR(1) NOT NULL,
	descricao VARCHAR(10) NOT NULL,
	realizada_em TIMESTAMP NOT NULL DEFAULT NOW(),
	CONSTRAINT fk_clientes_transacoes_id
		FOREIGN KEY (cliente_id) REFERENCES clientes(id)
);

DO $$
BEGIN
	INSERT INTO clientes (nome, limite, saldo)
	VALUES
        ('o barato sai caro', 1000 * 100, 0),
		('zan corp ltda', 800 * 100, 0),
		('les cruders', 10000 * 100, 0),
		('padaria joia de cocaia', 100000 * 100, 0),
		('kid mais', 5000 * 100, 0);
END;
$$;

CREATE INDEX idx_transacoes_realizada_em ON transacoes (realizada_em);
CREATE INDEX idx_transacoes_cliente_id ON transacoes (cliente_id);

CREATE OR REPLACE FUNCTION efetuar_transacao(
    clienteIdParam int,
    tipoParam varchar(1),
    valorParam int,
    descricaoParam varchar(10)
)
RETURNS TABLE (saldoRetorno int, limiteRetorno int) AS $$
DECLARE
    clientes clientes%rowtype;
    novoSaldo int;
    numeroLinhasAfetadas int;
BEGIN
    PERFORM * FROM clientes WHERE id = clienteIdParam FOR UPDATE;

    IF tipoParam = 'd' THEN
        novoSaldo := valorParam * -1;
    ELSE
        novoSaldo := valorParam;
    END IF;

    UPDATE clientes
    SET saldo = saldo + novoSaldo
    WHERE id = clienteIdParam
    AND (novoSaldo > 0 OR limite * -1 <= saldo + novoSaldo)
    RETURNING * INTO clientes;

    GET DIAGNOSTICS numeroLinhasAfetadas = ROW_COUNT;

    IF numeroLinhasAfetadas = 0 THEN
        RAISE EXCEPTION 'Limite indisponível';
    END IF;

    INSERT INTO transacoes (cliente_id, valor, tipo, descricao, realizada_em)
    VALUES (clienteIdParam, valorParam, tipoParam, descricaoParam, current_timestamp);

    RETURN QUERY SELECT clientes.saldo, clientes.limite;
END;
$$
LANGUAGE plpgsql;
