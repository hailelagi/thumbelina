alias Thumbelina.Internal

# dimensions: 2560 × 1600
# 2,316,459 bytes
{:ok, image} = Thumbelina.open("./example/mid-image.jpeg")

Benchee.run(
  %{
    "sync resize 2GiG" => fn -> Thumbelina.thumbnail(image, 50, 50) end,
    "async resize 2GiG" => fn ->
      Internal.cast(
        :thumbnail,
        self(),
        image.bytes,
        image.extension,
        50.0,
        50.0
      )
    end
  },
  time: 10,
  memory_time: 2
)
