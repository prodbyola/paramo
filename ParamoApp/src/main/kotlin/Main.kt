import androidx.compose.desktop.ui.tooling.preview.Preview
import androidx.compose.runtime.Composable
import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import androidx.compose.ui.unit.dp

import HelveticaFont
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.sp

@Composable
@Preview
fun App() {

    MaterialTheme(
        typography = Typography(
            defaultFontFamily = HelveticaFont
        ),
        colors = MaterialTheme.colors.copy(
            primary = Color(0xFF1DB3D6),
            secondary = Color(0xFF54C2BE)
        )
    ) {
        Box(
            modifier = Modifier.fillMaxSize().background(color = Color.White),
            contentAlignment = Alignment.BottomEnd
//            verticalArrangement = Arrangement.Center,
//            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Box(
                modifier = Modifier.fillMaxSize(),
                contentAlignment = Alignment.Center
            ){
                Column(
                    verticalArrangement = Arrangement.Center,
                    horizontalAlignment = Alignment.CenterHorizontally,
                    modifier = Modifier.border(
                        width = 1.dp,
                        color = Color(0x40000000),
                        shape = RoundedCornerShape(size = 10.dp),
                    )
                        .padding(1.dp)
                        .width(531.dp)
                        .height(284.dp)
                ) {
                    Image(
                        painter = painterResource("image/upload.png"),
                        contentDescription = "Upload Icon",
                        modifier = Modifier.size(48.dp)
                    )
                    Spacer(modifier = Modifier.height(24.dp))

                    Column(
                        modifier = Modifier.width(252.dp),
                        horizontalAlignment = Alignment.CenterHorizontally
                    ) {
                        Text(
                            text = "Select a file or drag and drop here",
                            fontSize = 13.sp,
                        )
                        Spacer(modifier = Modifier.height(12.dp))
                        Text(
                            text = "Do not attempt to compress already compressed file types like MP3, MP4 or archives.",
                            fontSize = 13.sp,
                            color = Color(0x66000000),
                            textAlign = TextAlign.Center
                        )
                        Spacer(modifier = Modifier.height(24.dp))
                        OutlinedButton(
                            onClick = {}
                        ){
                            Text(
                                "Select File".uppercase(),
                                fontSize = 10.sp,
                            )
                        }
                    }
                }
            }

            // Page footer
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .height(80.dp)
                    .background(color = Color(0xFFFBFDFE))
                    .padding(end = 56.dp),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.End
            ) {
                Button(
                    onClick = {},
                    colors = ButtonDefaults.buttonColors(backgroundColor = Color.Transparent),
                    elevation = null
                ) {
                    Text("Cancel")
                }
                Button(
                    onClick = {},
                    elevation = null,
                    enabled = false,
                ) {
                    Text("Upload")
                }
            }
        }
    }
}

fun main() = application {
    Window(
        title = "Paramo App",
        onCloseRequest = ::exitApplication,
    ) {
        App()
    }
}
