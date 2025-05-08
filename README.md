# 🌦️ Weather App

Un'applicazione full-stack per visualizzare le condizioni meteo attuali in varie città, utilizzando:

- **Backend**: Rust + Axum + Reqwest
- **Frontend**: React + Fetch API
- **API esterna**: [WeatherAPI.com](https://www.weatherapi.com/)

---

## 🚀 Funzionalità

- Visualizzazione meteo attuale (temperatura, umidità, descrizione) per città selezionate
- Frontend reattivo con possibilità di inserimento città
- Backend asincrono e performante con gestione del CORS

---

## 🛠️ Struttura del progetto

weather_api/  
├── src/  
│ ├── backend/ # Backend Rust  
│ │ ├── main.rs  
│ │ ├── handlers.rs  
│ │ └── weather.rs  
│ └── frontend/ # Frontend React  
├── .gitignore  
├── .env  
└── README.md  

---

## 🧪 Come eseguire

### ✅ Prerequisiti

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js + npm](https://nodejs.org/)
- Una chiave API da [weatherapi.com](https://www.weatherapi.com/)

### 🔐 Configura l’ambiente

Modifica il file `.env` in `src/backend/`:   
WEATHER_API_KEY=la_tua_api_key_qui

---

### ▶️ Avvia il backend

```cd weather_api```  
```cargo run --bin weather_api```

Il server sarà attivo su http://localhost:4000.

### ▶️ Avvia il frontend

```cd src/frontend```  
```npm install```  
```npm start```  

Il frontend sarà disponibile su http://localhost:3000.

---

### 💡 Note
Assicurati che il backend sia in esecuzione prima del frontend.  

Le richieste vengono effettuate da React a ```http://localhost:4000/weather.```




