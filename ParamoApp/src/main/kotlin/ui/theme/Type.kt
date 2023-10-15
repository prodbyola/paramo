import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.platform.Font

val HelveticaFont = FontFamily(
    Font(
        resource = "font/helvetica.ttf",
        weight = FontWeight.Normal,
        style = FontStyle.Normal
    ),
    Font(
        resource = "font/helvetica_bold.ttf",
        weight = FontWeight.Bold,
    ),
    Font(
        resource = "font/helvetica_light.ttf",
        weight = FontWeight.Light,
    ),
)