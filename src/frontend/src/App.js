import React, { useState } from 'react';
import './App.css';

function App() {
  const [weatherData, setWeatherData] = useState(null);
  const [cityInput, setCityInput] = useState('');
  const [city, setCity] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);  // ⬅️ nuovo stato per l’errore

  const fetchWeather = async (cityName) => {
    if (!cityName) return;
    setLoading(true);
    setError(null); // ⬅️ resetta l'errore
    try {
      const response = await fetch(`http://localhost:4000/weather?city=${cityName}`);
      if (!response.ok) {
        throw new Error(`Errore: ${response.status} ${response.statusText}`);
      }
      const data = await response.json();

      if (data.description === 'Errore') {
        throw new Error("Città non trovata o errore nella risposta.");
      }

      setWeatherData(data);
      setCity(cityName);
    } catch (err) {
      setWeatherData(null);
      setCity(cityName);
      setError(err.message); // ⬅️ registra l’errore
    } finally {
      setLoading(false);
    }
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    fetchWeather(cityInput.trim());
  };

  return (
    <div className="App">
      <h1>Meteo{city ? ` di ${city}` : ''}</h1>

      <form onSubmit={handleSubmit}>
        <input
          type="text"
          value={cityInput}
          onChange={(e) => setCityInput(e.target.value)}
          placeholder="Inserisci una città"
        />
        <button type="submit">Cerca</button>
      </form>

      {loading && <p>Caricamento dati meteo...</p>}

      {error && (
        <p style={{ color: 'red', fontWeight: 'bold' }}>
          Errore: {error}
        </p>
      )}

      {!loading && !error && weatherData && (
        <div>
          <p>Temperatura: {weatherData.temperature}°C</p>
          <p>Descrizione: {weatherData.description}</p>
          <p>Umidità: {weatherData.humidity}%</p>
        </div>
      )}
    </div>
  );
}

export default App;
