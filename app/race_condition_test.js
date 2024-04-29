//k6 run --vus 10 --iterations 10 ./race_condition_test.js

import { group } from "k6";
import http from "k6/http";

export default () => {
    group('Rota de Transação', () => {
        const params = {
            headers: { 
                'Content-Type': 'application/json'
            },
        };

        let body = {
            "valor": 50,
            "tipo": "c",
            "descricao": "descricaoa"
        };

        let res = http.post("http://localhost:9999/clientes/1/transacoes", JSON.stringify(body), params);
    });
}