image_path = Path.expand("archive/images/images")
{:ok, image} = Thumbelina.open(image_path)

Benchee.run(
  %{
    "resize" => fn ->
      Thumbelina.resize(image, 50, 50)
    end,
  time: 20,
  memory_time: 2
)
