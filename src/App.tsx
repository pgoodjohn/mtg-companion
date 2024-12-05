import "./App.css";

import CardsCollection from "./CardsCollection";
import NewCardForm from "./NewCardForm";

function App() {

  return (
    <main className="container mx-auto p-4">
        <NewCardForm />
      <div className="px-2">
        <p className="text-xl font-bold mt-4">Your Collection</p>
        <CardsCollection />
      </div>
    </main>
  );
}

export default App;
