cd app

#SOBE INFRA
sudo docker-compose up -d --build

echo "pausa de 1 min para startup da API"
sleep 60

#EXECUTA GATLING
sudo sh ../gatling/bin/gatling.sh -rm local -s RinhaBackendCrebitosSimulation \
    -rd "Rinha de Backend - 2024/Q1: Crébito - DeveloperArthur" \
    -rf ../gatling/results/DeveloperArthur \
    -sf ../gatling/user-files/simulations

#CALCULA RESULTADO FINAL
valorContrato=100000.0
SLARespostasOk=98.0
multaInconsistenciaSaldoLimiteUnidade=803.01

diretorio="../gatling/results/DeveloperArthur" 
arquivoStats=$(find $diretorio -name stats.json)
reportFile=$(find $diretorio -name index.html)
simulationFile=$(find $diretorio -name simulation.log)
reportDir=$(dirname $reportFile)

totalRequests=$(cat $arquivoStats | jq '.stats.numberOfRequests.total')
responsesOkMenos250ms=$(cat $arquivoStats | jq '.stats.group1.count')
porcentagemRespostasAceitaveis=$(python3 -c "print(round(${responsesOkMenos250ms} / ${totalRequests} * 100, 2))")
inconsistenciasSaldoLimite=$(grep "ConsistenciaSaldoLimite" $simulationFile | wc -l)
inconsistenciaTransacoesSaldo=$(grep "jmesPath(saldo.total).find.is" $simulationFile | wc -l)
multaSLA250ms=$(python3 -c "print(max(0.0, round(((${SLARespostasOk} - ${porcentagemRespostasAceitaveis}) * 1000), 2)))")
multaSLAInconsSaldo=$(python3 -c "print(round(((${inconsistenciasSaldoLimite} + ${inconsistenciaTransacoesSaldo}) * ${multaInconsistenciaSaldoLimiteUnidade}), 2))")
multaSLATotal=$(python3 -c "print(round(${multaSLA250ms} + ${multaSLAInconsSaldo}, 2))")
pagamento=$(python3 -c "print(max(0.0, round(${valorContrato} - ${multaSLATotal}, 2)))")

sudo echo -n "| DeveloperArthur " 
sudo echo -n "| USD ${multaSLA250ms} " 
sudo echo -n "| USD ${multaSLAInconsSaldo} " 
sudo echo -n "| USD ${multaSLATotal} " 
sudo echo -n "| **USD ${pagamento}** " 