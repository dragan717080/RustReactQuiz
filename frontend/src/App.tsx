import { About } from "./components/About";
import { Footer } from "./components/Footer";
import { Navbar } from "./components/Navbar";
import { ScrollToTop } from "./components/ScrollToTop";
import { Quiz } from "./components/Quiz";
import "./App.css";

function App() {
  return (
    <>
      <Navbar />
      <Quiz />
      <About />
      <Footer />
      <ScrollToTop />
    </>
  );
}

export default App;
