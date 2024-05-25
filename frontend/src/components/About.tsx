import pilot from "../assets/pilot.png";

export const About = () => {
  return (
    <section
      id="about"
      className="container pt-12 pb-24 sm:pt-6"
    >
      <div className="bg-muted/50 border rounded-lg py-12">
        <div className="px-6 flex flex-col-reverse md:flex-row gap-8 md:gap-12">
          <img
            src={pilot}
            alt=""
            className="w-[300px] object-contain rounded-lg"
          />
          <div className="bg-green-0 flex flex-col justify-between">
            <div className="pb-6">
              <h2 className="text-3xl md:text-4xl font-bold">
                <span className="bg-gradient-to-b from-primary/60 to-primary text-transparent bg-clip-text">
                  About Dragan
                </span>
              </h2>
              <p className="text-xl text-muted-foreground mt-4 md:-mb-4">
                With a background in freelance front-end development, nowadays I am mainly building full apps with Next.js or React or doing HackerRank challenges in Python/PHP/Elixir/Rust/C#.
                <br /><br />
                I am always looking for new opportunities to connect to improve my coding skills.
                <br /><br />
                My enjoyment is following tech trends and building highly optimized full stack apps (Rust on backend and Next.js on frontend).
                <br /><br />
                I am always open for collaboration.
              </p>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};
