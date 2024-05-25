import { useEffect, useRef, useState } from "react";
import { Button } from "./ui/button";
import { Card, CardHeader, CardTitle } from "./ui/card";
import {
  useQuery,
} from 'react-query'

interface Answer {
  text: string;
  is_correct: boolean | null;
}

export interface Question {
  category_name: string;
  question: string;
  answers: Answer[];
}

/**
 * Randomize array in-place using Durstenfeld shuffle algorithm.
 */
function shuffleArray(array) {
  for (let i = array.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      const t = array[i];
      array[i] = array[j];
      array[j] = t;
  }

  return array;
}

const fetchData = async (): Question[] => {
  const response = await fetch(`${import.meta.env.VITE_API_BASE_URL}/questions`);
  if (!response.ok) {
    throw new Error('There was an error');
  }

  return response.json();
};

export const Quiz = () => {
  const prevQuestions = useRef<Question[]>([]);

  const [currentStage, setCurrentStage] = useState<number>(1);
  const [selectedIndex, setSelectedIndex] = useState<number>(0);
  const [correctAnswers, setCorrectAnswers] = useState<number>(0);
  const [gameIsActive, setGameIsActive] = useState<boolean>(true);
  const [questions, setQuestions] = useState<Question[]>([]);

  type Correct = true | false | null;

  const [allAnswers, setAllAnswers] = useState<Correct[]>(Array.from({ length: 10 }).fill(null));

  const { data, error, isLoading, isError } = useQuery(['data'], fetchData);

  useEffect(() => {
    if (!isLoading && !isError && data && data !== prevQuestions.current) {
      prevQuestions.current = data;
      setQuestions(shuffleArray(data).slice(0, 10));
    }
  }, [data, isLoading, isError]);

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (isError) {
    return <div>Error: {error.message}</div>;
  }

  const startNewGame = (answers: Correct[]) => {
    setCurrentStage(1)
    setGameIsActive(false)
    setCorrectAnswers(answers.filter(answer => answer === true).length)
    // eslint-disable-next-line
    setAllAnswers(Array.from({ length: 10 }).map(item => null))
  }

  const reviewAnswer = () => {
    const isCorrect = questions[currentStage - 1].answers.findIndex(answer => answer.is_correct === true) === selectedIndex
    const answers = [...allAnswers]
    answers[currentStage - 1] = isCorrect;
    setAllAnswers(answers)

    if (Array.from({ length: 10 }).map((_, i) => i).includes(currentStage)) {
      setCurrentStage(currentStage + 1)
    } else {
      startNewGame(answers)
    }
  }

  return (
    <section className="container py-24 sm:py-32 relative">
      {gameIsActive && questions.length
        ? (
          <div className="grid lg:grid-cols-[1fr,1fr] gap-8 place-items-center">
            <div className="min-w-full">
              <h2 className="text-3xl md:text-4xl font-bold">
                <span className="bg-gradient-to-b from-primary/60 to-primary text-transparent bg-clip-text">
                  Question {currentStage} of 10
                </span>
              </h2>

              <p className="text-muted-foreground text-xl mt-4 mb-8 ">
                {questions[currentStage - 1].question.text}
              </p>
              <div className="flex flex-col gap-8 transition duration-300">
                {questions[currentStage - 1].answers.map((answer, index) =>
                (
                  <Card
                    key={answer.text}
                    className={selectedIndex == index ? "bg-primary text-primary-foreground hover:bg-primary/90" : ""}
                    onClick={() => setSelectedIndex(index)}
                  >
                    <CardHeader className="space-y-1 flex md:flex-row justify-start items-start gap-4">
                      <div>
                        <CardTitle>{answer.text}</CardTitle>
                      </div>
                    </CardHeader>
                  </Card>
                )
                )}
              </div>
              <Button className="w-full md:w-1/3 mt-10" onClick={reviewAnswer}>Next</Button>
              <div className="my-6 grid grid-cols-5 md:grid-cols-10 gap-3 w-full md:w-fit">
                {allAnswers.map((answer, index) => (
                  answer === null
                    ? (
                      <div className="rounded-full h-10 w-10 flex items-center justify-center border-3 border-white p-4" key={index} />
                    )
                    : answer === false
                      ? (
                        <div className="rounded-full h-10 w-10 flex items-center justify-center" key={index}>
                          <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="red" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="lucide lucide-circle-check">
                            <circle cx="12" cy="12" r="10" />
                            <path d="m9 12 2 2 4-4" />
                          </svg>
                        </div>
                      )
                      : (
                        <div className="rounded-full h-10 w-10 flex items-center justify-center" key={index}>
                          <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="#22C55E" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="lucide lucide-circle-check">
                            <circle cx="12" cy="12" r="10" />
                            <path d="m9 12 2 2 4-4" />
                          </svg>
                        </div>
                      )
                ))}
              </div>
            </div>
          </div>
        ) : (
          <div className="my-10">
            <div className="text-5xl my-10">
              You had {correctAnswers} correct answer{correctAnswers !== 1 && "s"}
            </div>
            <Button className="" onClick={() => setGameIsActive(true)}>
              Start New Game
            </Button>
          </div>
        )}
      <div className="shadow"></div>
    </section>
  );
};
