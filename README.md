# peopleCarApp

##To start Project

```sh
download repo go to carPeopleApi folder in terminal
Enter  in the terminal: **cargo run**
```

##Endpoint

````sh
    http://localhost:3030/calculateDisselUsageForDistance?yearOfProduction=**199**&distance=**10**&fuelUsagePer100KM=**2**

    *We GET:*
    ```json
    {
        "fuelUsage": 0.2
    }
    ```

    http://localhost:3030/probabilityOfUnitInjectorFail?vin=**sshfh**

    *_We GET:*

    ```json
    {
      "failProbability": 0.02
    }
    ```
````
